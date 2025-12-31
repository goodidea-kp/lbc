use leptos::prelude::{
    component, view, Children, Effect, Get, IntoView, NodeRef, OnAttribute, Set, Signal,
};
use wasm_bindgen::JsCast;

use crate::util::TestAttr;

/// Context signal used to close/open modals by ID from anywhere in the component tree.
/// Convention: write "<id>-close" to request closing a modal with id = <id>.
/// Convention: write "<id>-open" to request opening a modal with id = <id>.
pub type ModalCloserContext = leptos::prelude::RwSignal<String>;

fn is_valid_modal_id(id: &str) -> bool {
    if let Some(rest) = id.strip_prefix("id") {
        !rest.is_empty() && rest.chars().all(|ch| ch.is_ascii_digit())
    } else {
        false
    }
}

fn closer_key(id: &str) -> String {
    format!("{}-close", id)
}

fn base_class(extra: &str) -> String {
    if extra.trim().is_empty() {
        "modal".to_string()
    } else {
        format!("modal {}", extra)
    }
}

/// A classic modal overlay implemented with `<dialog>` (modern browsers).
///
/// Content is placed inside the "modal-content" div.
///
/// ID format requirement:
/// - The `id` must match the pattern `id[0-9]+`, for example: "id1", "id99".
/// - To close a modal via context, write "<id>-close" (e.g., "id1-close") into ModalCloserContext.
/// - To open a modal via context, write "<id>-open" (e.g., "id1-open") into ModalCloserContext.
///
/// Notes:
/// - SSR renders the dialog closed (no `open` attribute).
/// - On the client, we call `showModal()` / `close()` to control visibility.
/// - Click outside closes (backdrop click).
/// - Escape closes (via `cancel` event).
///
/// https://bulma.io/documentation/components/modal/
///
#[component]
pub fn Modal(
    /// A unique ID for this modal used together with ModalCloserContext ("<id>-close"/"<id>-open").
    id: String,

    /// Modal body content rendered inside "modal-content".
    children: Children,

    /// Trigger content; clicking it opens the modal.
    trigger: Children,

    /// Extra classes for the modal root.
    #[prop(optional, into)]
    classes: Signal<String>,
) -> impl IntoView {
    assert!(
        is_valid_modal_id(&id),
        "Modal id must match 'id[0-9]+' (e.g., id1, id99); got '{}'",
        id
    );

    let (is_active, set_is_active) = leptos::prelude::signal(false);

    // Optional closer context support
    let closer = leptos::prelude::use_context::<ModalCloserContext>();

    // Watch for external close/open requests
    if let Some(closer_signal) = closer.clone() {
        let id_clone = id.clone();
        Effect::new(move |_| {
            let action = closer_signal.get();
            if action.is_empty() {
                return;
            }

            if let Some((target_id, op)) = action.split_once('-') {
                if target_id == id_clone {
                    if op == "close" {
                        set_is_active.set(false);
                    } else if op == "open" {
                        set_is_active.set(true);
                    }
                    closer_signal.set(String::new());
                }
            }
        });
    }

    let class = {
        let classes = classes.clone();
        move || base_class(&classes.get())
    };

    // NodeRef to call showModal()/close() on the underlying <dialog>
    let dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

    // Keep the actual <dialog> open/closed in sync with is_active (client-side).
    Effect::new({
        let dialog_ref = dialog_ref.clone();
        move |_| {
            let active = is_active.get();
            let Some(dialog_el) = dialog_ref.get() else {
                return;
            };

            // Only meaningful in the browser; on SSR this won't run.
            let dialog: web_sys::HtmlDialogElement = dialog_el.into_any().unchecked_into();

            if active {
                // Avoid throwing if already open.
                if !dialog.open() {
                    let _ = dialog.show_modal();
                }
            } else if dialog.open() {
                dialog.close();
            }
        }
    });

    view! {
        <>
            <div on:click=move |_| set_is_active.set(true)>{trigger()}</div>

            <dialog
                node_ref=dialog_ref
                id=id.clone()
                class=move || class()
                // Backdrop click: if the click target is the <dialog> itself, user clicked outside content.
                on:click=move |ev| {
                    if let Some(target) = ev.target() {
                        if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                            if el.tag_name().to_ascii_lowercase() == "dialog" {
                                set_is_active.set(false);
                            }
                        }
                    }
                }
                // Escape key: close on cancel.
                on:cancel=move |ev| {
                    ev.prevent_default();
                    set_is_active.set(false);
                }
                // If something else closes the dialog, sync state.
                on:close=move |_| {
                    set_is_active.set(false);
                }
            >
                <div class="modal-background" on:click=move |_| set_is_active.set(false)></div>

                <div class="modal-content">
                    {children()}
                </div>

                <button
                    class="modal-close is-large"
                    aria_labelledby-label="close"
                    type="button"
                    on:click=move |_| set_is_active.set(false)
                ></button>
            </dialog>
        </>
    }
}

/// A modal with header, body and footer sections ("modal-card" variant), implemented with `<dialog>`.
///
/// ID format requirement:
/// - The `id` must match the pattern `id[0-9]+`, for example: "id1", "id99".
/// - To close a modal via context, write "<id>-close" (e.g., "id1-close") into ModalCloserContext.
/// - To open a modal via context, write "<id>-open" (e.g., "id1-open") into ModalCloserContext.
///
/// Notes:
/// - SSR renders the dialog closed (no `open` attribute).
/// - Click outside closes.
/// - Escape closes.
///
/// https://bulma.io/documentation/components/modal/
///
#[component]
pub fn ModalCard(
    /// A unique ID for this modal used together with ModalCloserContext ("<id>-close"/"<id>-open").
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
) -> impl IntoView {
    assert!(
        is_valid_modal_id(&id),
        "Modal id must match 'id[0-9]+' (e.g., id1, id99); got '{}'",
        id
    );

    let (is_active, set_is_active) = leptos::prelude::signal(false);

    // Optional closer context support
    let closer = leptos::prelude::use_context::<ModalCloserContext>();

    if let Some(closer_signal) = closer.clone() {
        let id_clone = id.clone();
        Effect::new(move |_| {
            let action = closer_signal.get();
            if action.is_empty() {
                return;
            }

            if let Some((target_id, op)) = action.split_once('-') {
                if target_id == id_clone {
                    if op == "close" {
                        set_is_active.set(false);
                    } else if op == "open" {
                        set_is_active.set(true);
                    }
                    closer_signal.set(String::new());
                }
            }
        });
    }

    let class = {
        let classes = classes.clone();
        move || base_class(&classes.get())
    };

    let dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

    Effect::new({
        let dialog_ref = dialog_ref.clone();
        move |_| {
            let active = is_active.get();
            let Some(dialog_el) = dialog_ref.get() else {
                return;
            };

            let dialog: web_sys::HtmlDialogElement = dialog_el.into_any().unchecked_into();

            if active {
                if !dialog.open() {
                    let _ = dialog.show_modal();
                }
            } else if dialog.open() {
                dialog.close();
            }
        }
    });

    view! {
        <>
            <div on:click=move |_| set_is_active.set(true)>{trigger()}</div>

            <dialog
                node_ref=dialog_ref
                id=id.clone()
                class=move || class()
                on:click=move |ev| {
                    if let Some(target) = ev.target() {
                        if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                            if el.tag_name().to_ascii_lowercase() == "dialog" {
                                set_is_active.set(false);
                            }
                        }
                    }
                }
                on:cancel=move |ev| {
                    ev.prevent_default();
                    set_is_active.set(false);
                }
                on:close=move |_| {
                    set_is_active.set(false);
                }
            >
                <div class="modal-background" on:click=move |_| set_is_active.set(false)></div>

                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{title.clone()}</p>
                        <button
                            class="delete"
                            aria_labelledby-label="close"
                            type="button"
                            on:click=move |_| set_is_active.set(false)
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
                    on:click=move |_| set_is_active.set(false)
                ></button>
            </dialog>
        </>
    }
}

/// Provide a ModalCloserContext to descendants.
/// Write "<id>-close" to the context to request closing a modal by ID.
/// Write "<id>-open" to the context to request opening a modal by ID.
#[component]
pub fn ModalCloserProvider(
    /// Initial action value; default empty.
    #[prop(optional, into)]
    initial: Signal<String>,
    children: Children,
) -> impl IntoView {
    let signal = leptos::prelude::RwSignal::new(initial.get());
    leptos::prelude::provide_context::<ModalCloserContext>(signal);
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
            <Modal id="id1".to_string() trigger=Box::new(|| view!{ <button>"Open"</button> }.into_any())>
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

    #[test]
    fn closer_key_formats_expected_suffix() {
        assert_eq!(super::closer_key("id1"), "id1-close");
    }

    #[test]
    #[should_panic(expected = "Modal id must match 'id[0-9]+'")]
    fn modal_rejects_invalid_id_format() {
        // Using an invalid id like "m1" should panic on creation/SSR render.
        let _ = view! {
            <Modal id="m1".to_string() trigger=Box::new(|| view!{ <button>"Open"</button> }.into_any())>
                <div class="box">"X"</div>
            </Modal>
        }
        .to_html();
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
