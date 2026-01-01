use leptos::callback::Callback;
use leptos::prelude::Callable;
use leptos::prelude::CustomAttribute;
use leptos::prelude::{
    component, view, Children, ClassAttribute, Effect, ElementChild, Get, GetUntracked,
    GlobalAttributes, IntoView, NodeRef, NodeRefAttribute, OnAttribute, Set, Signal, Update,
    WriteSignal,
};
use leptos::web_sys;
use std::collections::HashSet;
use wasm_bindgen::JsCast;

/// A controller for opening/closing modals from anywhere in the component tree.
///
/// This avoids the "single command slot" problem (commands being overwritten).
/// Internally it tracks a set of open modal IDs.
///
/// This controller allows multiple modals to be open at the same time.
/// If you want "only one modal open globally", implement that policy in your app
/// (e.g., call `close_all()` before `open(id)`), or add a separate controller type.
#[derive(Clone)]
pub struct ModalController {
    open_ids: leptos::prelude::RwSignal<HashSet<String>>,
}

impl ModalController {
    pub fn new() -> Self {
        Self {
            open_ids: leptos::prelude::RwSignal::new(HashSet::new()),
        }
    }

    /// Returns true if the modal with `id` is currently open.
    ///
    /// IMPORTANT: this must be a *reactive* read so Effects that call it re-run when the set changes.
    pub fn is_open(&self, id: &str) -> bool {
        let set = self.open_ids.get();
        set.contains(id)
    }

    /// Open a modal by id.
    pub fn open(&self, id: impl Into<String>) {
        let id = id.into();
        crate::lbc_debug_log!("[ModalController] open({})", id);
        self.open_ids.update(|set: &mut HashSet<String>| {
            set.insert(id);
        });
    }

    /// Close a modal by id.
    pub fn close(&self, id: impl AsRef<str>) {
        let id = id.as_ref();
        crate::lbc_debug_log!("[ModalController] close({})", id);
        self.open_ids.update(|set: &mut HashSet<String>| {
            set.remove(id);
        });
    }

    /// Close all modals.
    pub fn close_all(&self) {
        crate::lbc_debug_log!("[ModalController] close_all()");
        self.open_ids.set(HashSet::new());
    }
}

/// Context type for the modal controller.
pub type ModalControllerContext = ModalController;

fn base_class(extra: &str, is_active: bool) -> String {
    // Bulma's modal CSS expects `.modal.is-active` to be visible.
    // When using <dialog>, we still apply Bulma classes for styling, but we must
    // also add `is-active` while open so Bulma doesn't hide it.
    let mut base = if extra.trim().is_empty() {
        "modal".to_string()
    } else {
        format!("modal {}", extra)
    };

    if is_active {
        base.push_str(" is-active");
    }

    base
}

/// Try to focus a preferred element inside the dialog for accessibility:
/// - first element with `[data-lbc-dialog-focus]`
/// - otherwise focus the dialog itself
fn focus_dialog(dialog: &web_sys::HtmlDialogElement) {
    if let Ok(Some(el)) = dialog.query_selector("[data-lbc-dialog-focus]") {
        if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() {
            let _ = html.focus();
            return;
        }
    }
    let _ = dialog.focus();
}

fn close_dialog(dialog_ref: &NodeRef<leptos::html::Dialog>) {
    if let Some(dialog_el) = dialog_ref.get_untracked() {
        let dialog: web_sys::HtmlDialogElement =
            dialog_el.unchecked_into::<web_sys::HtmlDialogElement>();
        if dialog.open() {
            dialog.close();
        }
    }
}

/// Shared dialog behavior:
/// - sync `is_active` <-> `<dialog>` open state using showModal()/close()
/// - close on Escape (cancel)
/// - close on close event
/// - focus management on open (WCAG H102-friendly)
#[component]
fn DialogShell(
    id: String,
    #[prop(optional, into)]
    classes: Signal<String>,
    is_active: Signal<bool>,
    set_is_active: Callback<bool>,
    dialog_ref: NodeRef<leptos::html::Dialog>,
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let is_active = is_active.clone();
        move || base_class(&classes.get(), is_active.get())
    };

    // Keep the actual <dialog> open/closed in sync with is_active (client-side).
    Effect::new({
        let dialog_ref = dialog_ref.clone();
        let set_is_active = set_is_active.clone();
        let id_for_log = id.clone();
        move |_| {
            let active = is_active.get();
            crate::lbc_debug_log!("[DialogShell:{}] effect: is_active={}", id_for_log, active);

            let Some(dialog_el) = dialog_ref.get() else {
                crate::lbc_debug_log!(
                    "[DialogShell:{}] effect: dialog_ref not mounted yet",
                    id_for_log
                );
                return;
            };

            let dialog: web_sys::HtmlDialogElement = dialog_el.unchecked_into();

            crate::lbc_debug_log!(
                "[DialogShell:{}] effect: dialog.open() currently={}",
                id_for_log,
                dialog.open()
            );

            if active {
                if !dialog.open() {
                    crate::lbc_debug_log!("[DialogShell:{}] calling showModal()", id_for_log);
                    let res = dialog.show_modal();
                    if res.is_err() {
                        crate::lbc_debug_log!(
                            "[DialogShell:{}] showModal() returned Err",
                            id_for_log
                        );
                    }
                }
                // Ensure focus is moved into the dialog (WCAG H102).
                focus_dialog(&dialog);
            } else if dialog.open() {
                crate::lbc_debug_log!("[DialogShell:{}] calling close()", id_for_log);
                dialog.close();
            }

            if !dialog.open() && active {
                crate::lbc_debug_log!(
                    "[DialogShell:{}] dialog is not open but state says active; forcing state false",
                    id_for_log
                );
                set_is_active.run(false);
            }
        }
    });

    let controller = leptos::prelude::use_context::<ModalControllerContext>();

    let id_for_cancel = id.clone();
    let id_for_close = id.clone();

    // When the dialog closes, also clear the controller state for this id (if present),
    // so we don't keep stale "open" ids around.
    let controller_for_close = controller.clone();
    let id_for_controller_close = id.clone();

    // For cancel we explicitly close the dialog and sync state/controller.
    let dialog_ref_for_cancel = dialog_ref.clone();
    let set_is_active_for_cancel = set_is_active.clone();
    let controller_for_cancel = controller.clone();
    let id_for_controller_cancel = id.clone();

    view! {
        <>
            <style>
                r#"
                /* IMPORTANT:
                   Only show the dialog overlay when the native dialog is actually open.
                   This prevents "ghost" overlays when state says closed. */
                dialog.modal:not([open]) {
                    display: none !important;
                }

                /* Make <dialog class="modal"> behave like Bulma's full-screen modal container. */
                dialog.modal[open] {
                    position: fixed !important;
                    inset: 0 !important;
                    width: 100vw !important;
                    height: 100vh !important;

                    /* Center the inner Bulma modal content/card. */
                    display: flex !important;
                    align-items: center !important;
                    justify-content: center !important;

                    /* Neutralize native <dialog> chrome so Bulma's inner markup controls appearance. */
                    border: 0 !important;
                    outline: 0 !important;
                    box-shadow: none !important;
                    padding: 0 !important;
                    margin: 0 !important;
                    background: transparent !important;
                    color: inherit !important;
                    max-width: none !important;
                    max-height: none !important;

                    /* Some browsers apply native styling via appearance. */
                    -webkit-appearance: none;
                    appearance: none;
                }

                dialog.modal:focus,
                dialog.modal:focus-visible {
                    outline: 0 !important;
                    box-shadow: none !important;
                }

                /* Native backdrop (Bulma also renders .modal-background inside). */
                dialog.modal::backdrop {
                    background: rgba(10, 10, 10, 0.86);
                }
                "#
            </style>

            <dialog
                node_ref=dialog_ref
                id=id
                class=move || class()
                // Escape: close the dialog and sync state/controller.
                on:cancel=move |ev: web_sys::Event| {
                    crate::lbc_debug_log!("[DialogShell:{}] cancel (Escape) -> close", id_for_cancel);
                    ev.prevent_default();
                    close_dialog(&dialog_ref_for_cancel);
                    set_is_active_for_cancel.run(false);
                    if let Some(controller) = controller_for_cancel.as_ref() {
                        controller.close(&id_for_controller_cancel);
                    }
                }
                on:close=move |_ev: web_sys::Event| {
                    crate::lbc_debug_log!("[DialogShell:{}] close event -> state false", id_for_close);
                    set_is_active.run(false);

                    if let Some(controller) = controller_for_close.as_ref() {
                        controller.close(&id_for_controller_close);
                    }
                }
            >
                {children()}
            </dialog>
        </>
    }
}

#[component]
pub fn Modal(
    id: String,
    children: Children,
    trigger: Children,
    #[prop(optional, into)]
    classes: Signal<String>,
    #[prop(optional, into)]
    open: Option<Signal<bool>>,
    #[prop(optional)]
    set_open: Option<WriteSignal<bool>>,
) -> impl IntoView {
    let (internal_open, set_internal_open) = leptos::prelude::signal(false);
    let is_controlled = open.is_some() && set_open.is_some();

    let is_active: Signal<bool> = if let Some(open) = open {
        open
    } else {
        internal_open.into()
    };

    let controller = leptos::prelude::use_context::<ModalControllerContext>();

    // Local-only setter: updates controlled prop or internal signal, but does NOT touch controller.
    let set_local_open: Callback<bool> = {
        let set_open = set_open;
        let id_for_log = id.clone();
        Callback::new(move |v: bool| {
            crate::lbc_debug_log!("[Modal:{}] set_local_open({})", id_for_log, v);
            if is_controlled {
                if let Some(set_open) = set_open {
                    set_open.set(v);
                }
            } else {
                set_internal_open.set(v);
            }
        })
    };

    // If a controller exists and we're uncontrolled, the controller is the source of truth.
    if let Some(controller) = controller.clone() {
        let id_clone = id.clone();
        let set_local_open = set_local_open.clone();
        Effect::new(move |_| {
            if is_controlled {
                return;
            }
            let should_be_open = controller.is_open(&id_clone);
            crate::lbc_debug_log!(
                "[Modal:{}] controller sync effect: should_be_open={}",
                id_clone,
                should_be_open
            );
            set_local_open.run(should_be_open);
        });
    }

    let dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

    let close_action: Callback<()> = {
        let id = id.clone();
        let controller = controller.clone();
        let set_local_open = set_local_open.clone();
        let dialog_ref = dialog_ref.clone();
        Callback::new(move |_| {
            crate::lbc_debug_log!("[Modal:{}] close_action()", id);
            close_dialog(&dialog_ref);

            if !is_controlled {
                if let Some(controller) = controller.as_ref() {
                    controller.close(&id);
                    return;
                }
            }
            set_local_open.run(false);
            if let Some(controller) = controller.as_ref() {
                controller.close(&id);
            }
        })
    };

    let bg_close = close_action.clone();
    let close_btn_close = close_action.clone();

    view! {
        <>
            {trigger()}

            <DialogShell
                id=id
                classes=classes
                is_active=is_active
                set_is_active=set_local_open.clone()
                dialog_ref=dialog_ref
            >
                // Backdrop click should close reliably.
                <div class="modal-background" on:click=move |_ev: web_sys::MouseEvent| bg_close.run(())></div>

                <div class="modal-content">
                    {children()}
                </div>

                <button
                    class="modal-close is-large"
                    aria_labelledby-label="close"
                    type="button"
                    on:click=move |_ev: web_sys::MouseEvent| close_btn_close.run(())
                ></button>
            </DialogShell>
        </>
    }
}

#[component]
pub fn ModalCard(
    id: String,
    title: String,
    body: Children,
    footer: Children,
    trigger: Children,
    #[prop(optional, into)]
    classes: Signal<String>,
    #[prop(optional, into)]
    open: Option<Signal<bool>>,
    #[prop(optional)]
    set_open: Option<WriteSignal<bool>>,
) -> impl IntoView {
    let (internal_open, set_internal_open) = leptos::prelude::signal(false);
    let is_controlled = open.is_some() && set_open.is_some();

    let is_active: Signal<bool> = if let Some(open) = open {
        open
    } else {
        internal_open.into()
    };

    let controller = leptos::prelude::use_context::<ModalControllerContext>();

    let set_local_open: Callback<bool> = {
        let set_open = set_open;
        let id_for_log = id.clone();
        Callback::new(move |v: bool| {
            crate::lbc_debug_log!("[ModalCard:{}] set_local_open({})", id_for_log, v);
            if is_controlled {
                if let Some(set_open) = set_open {
                    set_open.set(v);
                }
            } else {
                set_internal_open.set(v);
            }
        })
    };

    if let Some(controller) = controller.clone() {
        let id_clone = id.clone();
        let set_local_open = set_local_open.clone();
        Effect::new(move |_| {
            if is_controlled {
                return;
            }
            let should_be_open = controller.is_open(&id_clone);
            crate::lbc_debug_log!(
                "[ModalCard:{}] controller sync effect: should_be_open={}",
                id_clone,
                should_be_open
            );
            set_local_open.run(should_be_open);
        });
    }

    let dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

    let close_action: Callback<()> = {
        let id = id.clone();
        let controller = controller.clone();
        let set_local_open = set_local_open.clone();
        let dialog_ref = dialog_ref.clone();
        Callback::new(move |_| {
            crate::lbc_debug_log!("[ModalCard:{}] close_action()", id);
            close_dialog(&dialog_ref);

            if !is_controlled {
                if let Some(controller) = controller.as_ref() {
                    controller.close(&id);
                    return;
                }
            }
            set_local_open.run(false);
            if let Some(controller) = controller.as_ref() {
                controller.close(&id);
            }
        })
    };

    let bg_close = close_action.clone();
    let delete_btn_close = close_action.clone();
    let close_btn_close = close_action.clone();

    view! {
        <>
            {trigger()}

            <DialogShell
                id=id
                classes=classes
                is_active=is_active
                set_is_active=set_local_open.clone()
                dialog_ref=dialog_ref
            >
                <div class="modal-background" on:click=move |_ev: web_sys::MouseEvent| bg_close.run(())></div>

                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title" tabindex="-1" data-lbc-dialog-focus="true">{title.clone()}</p>
                        <button
                            class="delete"
                            aria_labelledby-label="close"
                            type="button"
                            on:click=move |_ev: web_sys::MouseEvent| delete_btn_close.run(())
                        ></button>
                    </header>

                    <section class="modal-card-body">
                        {body()}
                    </section>

                    <footer class="modal-card-foot">
                        {footer()}
                    </footer>
                </div>

                <button
                    class="modal-close is-large"
                    aria_labelledby-label="close"
                    type="button"
                    on:click=move |_ev: web_sys::MouseEvent| close_btn_close.run(())
                ></button>
            </DialogShell>
        </>
    }
}

#[component]
pub fn ModalControllerProvider(children: Children) -> impl IntoView {
    let controller = ModalController::new();
    crate::lbc_debug_log!("[ModalControllerProvider] providing ModalControllerContext");
    leptos::prelude::provide_context::<ModalControllerContext>(controller);
    view! { {children()} }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::IntoAny;
    use leptos::prelude::RenderHtml;

    #[test]
    fn modal_renders_base_class_and_children() {
        let html = view! {
            <Modal id="any-id".to_string() trigger=Box::new(|| view!{ <button>"Open"</button> }.into_any())>
                <div class="box">"Hello"</div>
            </Modal>
        }
        .to_html();

        assert!(
            html.contains(r#"<dialog"#) && html.contains(r#"class="modal""#),
            "expected <dialog> with base 'modal' class, got: {}",
            html
        );
        assert!(html.contains("Hello"));
    }

    #[test]
    fn modal_card_renders_sections() {
        let html = view! {
            <ModalCard id="id2".to_string() title="Title".to_string()
                trigger=Box::new(|| view!{ <button>"Open"</button> }.into_any())
                body=Box::new(|| view!{ <p>"Body"</p> }.into_any())
                footer=Box::new(|| view!{ <button>"OK"</button> }.into_any())
            />
        }
        .to_html();

        assert!(html.contains("modal-card"));
        assert!(html.contains("Title"));
        assert!(html.contains("Body"));
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use leptos::prelude::IntoAny;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn trigger() -> Children {
        Box::new(|| view! { <button>"Open"</button> }.into_any())
    }

    #[wasm_bindgen_test]
    fn modal_renders_as_dialog() {
        let html = view! {
            <Modal
                id="id1".to_string()
                trigger=trigger()
                classes=""
            >
                <div class="box">"Hello"</div>
            </Modal>
        }
        .to_html();

        assert!(html.contains("<dialog") && html.contains(r#"class="modal""#));
    }
}
