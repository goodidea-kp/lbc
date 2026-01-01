use leptos::prelude::CustomAttribute;
use leptos::prelude::{
    component, view, Children, ClassAttribute, Effect, ElementChild, Get, GlobalAttributes,
    IntoView, NodeRef, NodeRefAttribute, OnAttribute, Set, Signal, WriteSignal,
};
use leptos::web_sys;
use std::collections::HashSet;
use std::sync::Arc;
use wasm_bindgen::JsCast;

/// A controller for opening/closing modals from anywhere in the component tree.
///
/// This avoids the "single command slot" problem (commands being overwritten).
/// Internally it tracks a set of open modal IDs.
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
    pub fn is_open(&self, id: &str) -> bool {
        self.open_ids.with(|set| set.contains(id))
    }

    /// Open a modal by id.
    pub fn open(&self, id: impl Into<String>) {
        let id = id.into();
        self.open_ids.update(|set| {
            set.insert(id);
        });
    }

    /// Close a modal by id.
    pub fn close(&self, id: impl AsRef<str>) {
        let id = id.as_ref();
        self.open_ids.update(|set| {
            set.remove(id);
        });
    }

    /// Close all modals.
    pub fn close_all(&self) {
        self.open_ids.set(HashSet::new());
    }
}

/// Context type for the modal controller.
pub type ModalControllerContext = ModalController;

fn base_class(extra: &str) -> String {
    if extra.trim().is_empty() {
        "modal".to_string()
    } else {
        format!("modal {}", extra)
    }
}

/// Shared dialog behavior:
/// - sync `is_active` <-> `<dialog>` open state using showModal()/close()
/// - close on backdrop click
/// - close on Escape (cancel)
/// - close on close event
#[component]
fn DialogShell(
    id: String,
    #[prop(optional, into)]
    classes: Signal<String>,
    is_active: Signal<bool>,
    set_is_active: Arc<dyn Fn(bool) + Send + Sync>,
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class(&classes.get())
    };

    let dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

    // Keep the actual <dialog> open/closed in sync with is_active (client-side).
    Effect::new({
        let dialog_ref = dialog_ref.clone();
        let set_is_active = set_is_active.clone();
        move |_| {
            let active = is_active.get();
            let Some(dialog_el) = dialog_ref.get() else {
                return;
            };

            // Cast the underlying DOM element to HtmlDialogElement.
            let dialog: web_sys::HtmlDialogElement = dialog_el.unchecked_into();

            if active {
                // Avoid throwing if already open.
                if !dialog.open() {
                    let _ = dialog.show_modal();
                }
            } else if dialog.open() {
                dialog.close();
            }

            // If the dialog was closed by the browser (rare here), keep state consistent.
            // (This is mostly defensive; on:close also handles it.)
            if !dialog.open() && active {
                (set_is_active)(false);
            }
        }
    });

    view! {
        <dialog
            node_ref=dialog_ref
            id=id
            class=move || class()
            // Backdrop click: if the click target is the <dialog> itself, user clicked outside content.
            on:click=move |ev: web_sys::MouseEvent| {
                if let Some(target) = ev.target() {
                    if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                        if el.tag_name().to_ascii_lowercase() == "dialog" {
                            (set_is_active)(false);
                        }
                    }
                }
            }
            // Escape key: close on cancel.
            on:cancel=move |ev: web_sys::Event| {
                ev.prevent_default();
                (set_is_active)(false);
            }
            // If something else closes the dialog, sync state.
            on:close=move |_ev: web_sys::Event| {
                (set_is_active)(false);
            }
        >
            {children()}
        </dialog>
    }
}

/// A classic modal overlay implemented with `<dialog>` (modern browsers).
///
/// Content is placed inside the "modal-content" div.
///
/// Notes:
/// - SSR renders the dialog closed (no `open` attribute).
/// - On the client, we call `showModal()` / `close()` to control visibility.
/// - Click outside closes (backdrop click).
/// - Escape closes (via `cancel` event).
/// - External open/close is supported via `ModalController` context.
/// - Can be controlled via `open`/`set_open` props; otherwise uses internal state.
///
/// https://bulma.io/documentation/components/modal/
///
#[component]
pub fn Modal(
    /// A unique ID for this modal (used by ModalController).
    id: String,

    /// Modal body content rendered inside "modal-content".
    children: Children,

    /// Trigger content; clicking it opens the modal.
    trigger: Children,

    /// Extra classes for the modal root.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Controlled open state. If provided together with `set_open`, the modal becomes controlled.
    #[prop(optional, into)]
    open: Option<Signal<bool>>,

    /// Controlled setter. If provided together with `open`, the modal becomes controlled.
    #[prop(optional)]
    set_open: Option<WriteSignal<bool>>,
) -> impl IntoView {
    // Internal state (used when not controlled).
    let (internal_open, set_internal_open) = leptos::prelude::signal(false);

    let is_controlled = open.is_some() && set_open.is_some();

    let is_active: Signal<bool> = if let Some(open) = open {
        open
    } else {
        internal_open.into()
    };

    let controller = leptos::prelude::use_context::<ModalControllerContext>();

    let set_is_active: Arc<dyn Fn(bool) + Send + Sync> = {
        let id = id.clone();
        let controller = controller.clone();
        let set_open = set_open;
        Arc::new(move |v: bool| {
            if is_controlled {
                if let Some(set_open) = set_open {
                    set_open.set(v);
                }
            } else {
                set_internal_open.set(v);
            }

            // Keep controller in sync if present.
            if let Some(controller) = controller.as_ref() {
                if v {
                    controller.open(id.clone());
                } else {
                    controller.close(&id);
                }
            }
        })
    };

    // If a controller exists, let it drive the open state (unless controlled by props).
    if let Some(controller) = controller.clone() {
        let id_clone = id.clone();
        let set_is_active = set_is_active.clone();
        Effect::new(move |_| {
            if is_controlled {
                return;
            }
            let should_be_open = controller.is_open(&id_clone);
            (set_is_active)(should_be_open);
        });
    }

    view! {
        <>
            <div on:click=move |_| (set_is_active)(true)>{trigger()}</div>

            <DialogShell
                id=id
                classes=classes
                is_active=is_active
                set_is_active=set_is_active.clone()
            >
                <div class="modal-background" on:click=move |_ev: web_sys::MouseEvent| (set_is_active)(false)></div>

                <div class="modal-content">
                    {children()}
                </div>

                <button
                    class="modal-close is-large"
                    aria_labelledby-label="close"
                    type="button"
                    on:click=move |_ev: web_sys::MouseEvent| (set_is_active)(false)
                ></button>
            </DialogShell>
        </>
    }
}

/// A modal with header, body and footer sections ("modal-card" variant), implemented with `<dialog>`.
///
/// Notes:
/// - SSR renders the dialog closed (no `open` attribute).
/// - Click outside closes.
/// - Escape closes.
/// - External open/close is supported via `ModalController` context.
/// - Can be controlled via `open`/`set_open` props; otherwise uses internal state.
///
/// https://bulma.io/documentation/components/modal/
///
#[component]
pub fn ModalCard(
    /// A unique ID for this modal (used by ModalController).
    id: String,

    /// Title text shown in the modal-card header.
    title: String,

    /// Content placed in the modal-card body.
    body: Children,

    /// Content placed into the modal-card footer.
    footer: Children,

    /// Trigger content; clicking it opens the modal.
    trigger: Children,

    /// Extra classes for the modal root.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Controlled open state. If provided together with `set_open`, the modal becomes controlled.
    #[prop(optional, into)]
    open: Option<Signal<bool>>,

    /// Controlled setter. If provided together with `open`, the modal becomes controlled.
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

    let set_is_active: Arc<dyn Fn(bool) + Send + Sync> = {
        let id = id.clone();
        let controller = controller.clone();
        let set_open = set_open;
        Arc::new(move |v: bool| {
            if is_controlled {
                if let Some(set_open) = set_open {
                    set_open.set(v);
                }
            } else {
                set_internal_open.set(v);
            }

            if let Some(controller) = controller.as_ref() {
                if v {
                    controller.open(id.clone());
                } else {
                    controller.close(&id);
                }
            }
        })
    };

    if let Some(controller) = controller.clone() {
        let id_clone = id.clone();
        let set_is_active = set_is_active.clone();
        Effect::new(move |_| {
            if is_controlled {
                return;
            }
            let should_be_open = controller.is_open(&id_clone);
            (set_is_active)(should_be_open);
        });
    }

    view! {
        <>
            <div on:click=move |_| (set_is_active)(true)>{trigger()}</div>

            <DialogShell
                id=id
                classes=classes
                is_active=is_active
                set_is_active=set_is_active.clone()
            >
                <div class="modal-background" on:click=move |_ev: web_sys::MouseEvent| (set_is_active)(false)></div>

                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{title.clone()}</p>
                        <button
                            class="delete"
                            aria_labelledby-label="close"
                            type="button"
                            on:click=move |_ev: web_sys::MouseEvent| (set_is_active)(false)
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
                    on:click=move |_ev: web_sys::MouseEvent| (set_is_active)(false)
                ></button>
            </DialogShell>
        </>
    }
}

/// Provide a `ModalController` to descendants.
///
/// Descendants can call `use_context::<ModalControllerContext>()` and then:
/// - `controller.open("my-modal")`
/// - `controller.close("my-modal")`
/// - `controller.close_all()`
#[component]
pub fn ModalControllerProvider(children: Children) -> impl IntoView {
    let controller = ModalController::new();
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
