use leptos::prelude::CustomAttribute;
use leptos::prelude::{
    component, view, Children, ClassAttribute, Effect, ElementChild, Get, GlobalAttributes,
    IntoView, NodeRef, NodeRefAttribute, OnAttribute, Set, Signal, Update, WriteSignal,
};
use leptos::web_sys;
use std::collections::HashSet;
use std::sync::Arc;
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

/// Shared dialog behavior:
/// - sync `is_active` <-> `<dialog>` open state using showModal()/close()
/// - close on backdrop click
/// - close on Escape (cancel)
/// - close on close event
/// - focus management on open (WCAG H102-friendly)
#[component]
fn DialogShell(
    id: String,
    #[prop(optional, into)]
    classes: Signal<String>,
    is_active: Signal<bool>,
    set_is_active: Arc<dyn Fn(bool) + Send + Sync>,
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
                (set_is_active)(false);
            }
        }
    });

    let controller = leptos::prelude::use_context::<ModalControllerContext>();

    let on_click_setter = set_is_active.clone();
    let on_close_setter = set_is_active.clone();

    let id_for_click = id.clone();
    let id_for_cancel = id.clone();
    let id_for_close = id.clone();

    // When the dialog closes, also clear the controller state for this id (if present),
    // so we don't keep stale "open" ids around.
    let controller_for_close = controller.clone();
    let id_for_controller_close = id.clone();

    view! {
        <dialog
            node_ref=dialog_ref
            id=id
            class=move || class()
            on:click=move |ev: web_sys::MouseEvent| {
                if let Some(target) = ev.target() {
                    if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                        if el.tag_name().to_ascii_lowercase() == "dialog" {
                            crate::lbc_debug_log!("[DialogShell:{}] backdrop click -> close", id_for_click);
                            (on_click_setter)(false);
                        }
                    }
                }
            }
            // Let the browser handle Escape-to-close. We only log here.
            // We'll sync state in `on:close`.
            on:cancel=move |_ev: web_sys::Event| {
                crate::lbc_debug_log!("[DialogShell:{}] cancel (Escape) observed", id_for_cancel);
            }
            on:close=move |_ev: web_sys::Event| {
                crate::lbc_debug_log!("[DialogShell:{}] close event -> state false", id_for_close);
                (on_close_setter)(false);

                if let Some(controller) = controller_for_close.as_ref() {
                    controller.close(&id_for_controller_close);
                }
            }
        >
            {children()}
        </dialog>
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
    let set_local_open: Arc<dyn Fn(bool) + Send + Sync> = {
        let set_open = set_open;
        let id_for_log = id.clone();
        Arc::new(move |v: bool| {
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
            (set_local_open)(should_be_open);
        });
    }

    let close_action: Arc<dyn Fn() + Send + Sync> = {
        let id = id.clone();
        let controller = controller.clone();
        let set_local_open = set_local_open.clone();
        Arc::new(move || {
            crate::lbc_debug_log!("[Modal:{}] close_action()", id);
            if !is_controlled {
                if let Some(controller) = controller.as_ref() {
                    controller.close(&id);
                    return;
                }
            }
            (set_local_open)(false);
            if let Some(controller) = controller.as_ref() {
                controller.close(&id);
            }
        })
    };

    let dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

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
                <div class="modal-background" on:click=move |_ev: web_sys::MouseEvent| (bg_close)()></div>

                <div class="modal-content">
                    {children()}
                </div>

                <button
                    class="modal-close is-large"
                    aria_labelledby-label="close"
                    type="button"
                    on:click=move |_ev: web_sys::MouseEvent| (close_btn_close)()
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

    let set_local_open: Arc<dyn Fn(bool) + Send + Sync> = {
        let set_open = set_open;
        let id_for_log = id.clone();
        Arc::new(move |v: bool| {
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
            (set_local_open)(should_be_open);
        });
    }

    let close_action: Arc<dyn Fn() + Send + Sync> = {
        let id = id.clone();
        let controller = controller.clone();
        let set_local_open = set_local_open.clone();
        Arc::new(move || {
            crate::lbc_debug_log!("[ModalCard:{}] close_action()", id);
            if !is_controlled {
                if let Some(controller) = controller.as_ref() {
                    controller.close(&id);
                    return;
                }
            }
            (set_local_open)(false);
            if let Some(controller) = controller.as_ref() {
                controller.close(&id);
            }
        })
    };

    let dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

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
                <div class="modal-background" on:click=move |_ev: web_sys::MouseEvent| (bg_close)()></div>

                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title" tabindex="-1" data-lbc-dialog-focus="true">{title.clone()}</p>
                        <button
                            class="delete"
                            aria_labelledby-label="close"
                            type="button"
                            on:click=move |_ev: web_sys::MouseEvent| (delete_btn_close)()
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
                    on:click=move |_ev: web_sys::MouseEvent| (close_btn_close)()
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
            "expected <dialog> with base 'modal' class; got: {}",
            html
        );
        assert!(
            html.contains("Hello"),
            "expected modal children rendered; got: {}",
            html
        );
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

        assert!(
            html.contains("modal-card"),
            "expected modal-card structure; got: {}",
            html
        );
        assert!(html.contains("Title"), "expected title; got: {}", html);
        assert!(html.contains("Body"), "expected body; got: {}", html);
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

        assert!(
            html.contains("<dialog") && html.contains(r#"class="modal""#),
            "expected <dialog class=\"modal\">; got: {}",
            html
        );
    }
}
