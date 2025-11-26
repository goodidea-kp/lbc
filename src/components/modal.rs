use leptos::prelude::{
    AriaAttributes, Children, ClassAttribute, CustomAttribute, ElementChild, Get, GlobalAttributes,
    IntoView, OnAttribute, Set, Signal, component, view,
};

use crate::util::TestAttr;

/// Context signal used to close modals by ID from anywhere in the component tree.
/// Convention: write "<id>-close" to request closing a modal with id = <id>.
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

/// A classic modal overlay. Content is placed inside the "modal-content" div.
///
/// ID format requirement:
/// - The `id` must match the pattern `id[0-9]+`, for example: "id1", "id99".
/// - To close a modal via context, write "<id>-close" (e.g., "id1-close") into ModalCloserContext.
///
/// https://bulma.io/documentation/components/modal/
#[component]
pub fn Modal(
    /// A unique ID for this modal used together with ModalCloserContext ("<id>-close").
    id: String,

    /// Modal body content rendered inside "modal-content".
    children: Children,

    /// Trigger content; clicking it opens the modal.
    trigger: Children,

    /// Extra classes for the modal root.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test attribute (renders as data-* attribute) on the modal root.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    assert!(
        is_valid_modal_id(&id),
        "Modal id must match 'id[0-9]+' (e.g., id1, id99); got '{}'",
        id
    );
    let (is_active, set_is_active) = leptos::prelude::signal(false);

    // Optional closer context support
    let closer = leptos::prelude::use_context::<ModalCloserContext>();

    // Watch for external close requests
    if let Some(closer_sig) = closer.clone() {
        // clone the id so we don't move it into the effect closure
        let id_clone = id.clone();
        leptos::prelude::Effect::new(move |_| {
            let action = closer_sig.get();
            if !action.is_empty() {
                if let Some((target_id, op)) = action.split_once('-') {
                    if target_id == id_clone && op == "close" {
                        set_is_active.set(false);
                        // reset context to avoid re-trigger
                        closer_sig.set(String::new());
                    }
                }
            }
        });
    }

    let class = {
        let classes = classes.clone();
        move || {
            let mut cls = base_class(&classes.get());
            if is_active.get() {
                cls.push_str(" is-active");
            }
            cls
        }
    };

    let open_click = move |_| set_is_active.set(true);
    let close_click = move |_| set_is_active.set(false);

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <>
            <div on:click=open_click>{trigger()}</div>
            <div
                id=id.clone()
                class=move || class()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                <div class="modal-background" on:click=close_click></div>
                <div class="modal-content">
                    {children()}
                </div>
                <button class="modal-close is-large" aria-label="close" on:click=close_click></button>
            </div>
        </>
    }
}

/// A modal with header, body and footer sections ("modal-card" variant).
///
/// ID format requirement:
/// - The `id` must match the pattern `id[0-9]+`, for example: "id1", "id99".
/// - To close a modal via context, write "<id>-close" (e.g., "id1-close") into ModalCloserContext.
///
/// https://bulma.io/documentation/components/modal/
#[component]
pub fn ModalCard(
    /// A unique ID for this modal used together with ModalCloserContext ("<id>-close").
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

    /// Optional test attribute (renders as data-* attribute) on the modal root.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    assert!(
        is_valid_modal_id(&id),
        "Modal id must match 'id[0-9]+' (e.g., id1, id99); got '{}'",
        id
    );
    let (is_active, set_is_active) = leptos::prelude::signal(false);

    // Optional closer context support
    let closer = leptos::prelude::use_context::<ModalCloserContext>();

    if let Some(closer_sig) = closer.clone() {
        // clone the id so we don't move it into the effect closure
        let id_clone = id.clone();
        leptos::prelude::Effect::new(move |_| {
            let action = closer_sig.get();
            if !action.is_empty() {
                if let Some((target_id, op)) = action.split_once('-') {
                    if target_id == id_clone && op == "close" {
                        set_is_active.set(false);
                        closer_sig.set(String::new());
                    }
                }
            }
        });
    }

    let class = {
        let classes = classes.clone();
        move || {
            let mut cls = base_class(&classes.get());
            if is_active.get() {
                cls.push_str(" is-active");
            }
            cls
        }
    };

    let open_click = move |_| set_is_active.set(true);
    let close_click = move |_| set_is_active.set(false);

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <>
            <div on:click=open_click>{trigger()}</div>
            <div
                id=id.clone()
                class=move || class()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                <div class="modal-background" on:click=close_click></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{title.clone()}</p>
                        <button class="delete" aria-label="close" on:click=close_click></button>
                    </header>
                    <section class="modal-card-body">
                        {body()}
                    </section>
                    <footer class="modal-card-foot">
                        {footer()}
                    </footer>
                </div>
                <button class="modal-close is-large" aria-label="close" on:click=close_click></button>
            </div>
        </>
    }
}

/// Provide a ModalCloserContext to descendants.
/// Write "<id>-close" to the context to request closing a modal by ID.
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
        }.to_html();

        assert!(
            html.contains(r#"class="modal""#),
            "expected base 'modal' class; got: {}",
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
        }.to_html();
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
    fn modal_renders_test_id() {
        let html = view! {
            <Modal
                id="id1".to_string()
                trigger=trigger()
                classes="is-active"
                test_attr=TestAttr::test_id("modal-test")
            >
                <div class="box">"Hello"</div>
            </Modal>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="modal-test""#),
            "expected data-testid attribute on Modal; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn modal_no_test_attr_when_not_provided() {
        let html = view! {
            <Modal id="id1".to_string() trigger=trigger()>
                <div class="box">"Hello"</div>
            </Modal>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute on Modal when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn modal_card_renders_test_id() {
        let html = view! {
            <ModalCard
                id="id2".to_string()
                title="Title".to_string()
                trigger=trigger()
                body=Box::new(|| view!{ <p>"Body"</p> }.into_any())
                footer=Box::new(|| view!{ <button>"OK"</button> }.into_any())
                test_attr=TestAttr::test_id("modal-card-test")
            />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="modal-card-test""#),
            "expected data-testid attribute on ModalCard; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn modal_card_no_test_attr_when_not_provided() {
        let html = view! {
            <ModalCard
                id="id2".to_string()
                title="Title".to_string()
                trigger=trigger()
                body=Box::new(|| view!{ <p>"Body"</p> }.into_any())
                footer=Box::new(|| view!{ <button>"OK"</button> }.into_any())
            />
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute on ModalCard when not provided; got: {}",
            html
        );
    }
}
