/*!
Leptos version of Bulma Delete element.

Bulma docs: https://bulma.io/documentation/elements/delete/
*/
use leptos::callback::Callable;
use leptos::ev::MouseEvent;
use leptos::prelude::{
    AnyView, Callback, Children, ClassAttribute, CustomAttribute, ElementChild, Get, IntoAny,
    OnAttribute, Signal, component, view,
};

use crate::util::TestAttr;

/// A versatile delete cross.
#[component]
pub fn Delete(
    /// Additional CSS classes to append to the base "delete" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (button, a, span, div)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,
    /// Child content to render inside (usually empty for Bulma delete, but allowed)
    #[prop(optional)]
    children: Option<Children>,
    /// Optional click handler passed through to the rendered element.
    #[prop(optional)]
    on_click: Option<Callback<MouseEvent>>,
    /// Optional test attribute (renders as data-* attribute) on the rendered element.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (for example, `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> AnyView {
    // Build class attribute: "delete [extra classes]"
    let mut class_attr = String::from("delete");

    if let Some(extra) = classes {
        let extra_val = extra.get();
        if !extra_val.trim().is_empty() {
            class_attr.push(' ');
            class_attr.push_str(extra_val.trim());
        }
    }

    let tag_name = tag
        .as_ref()
        .map(|tag| tag.get().to_lowercase())
        .unwrap_or_else(|| "button".to_string());

    // Render children only if provided; otherwise render nothing.
    let content = match children {
        Some(children) => children(),
        None => view! {}.into_any(),
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    match tag_name.as_str() {
        "a" => view! {
            <a
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
                on:click=move |ev| {
                    if let Some(on_click) = on_click {
                        on_click.run(ev);
                    }
                }
            >
                {content}
            </a>
        }
        .into_any(),
        "span" => view! {
            <span
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
                on:click=move |ev| {
                    if let Some(on_click) = on_click {
                        on_click.run(ev);
                    }
                }
            >
                {content}
            </span>
        }
        .into_any(),
        "div" => view! {
            <div
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
                on:click=move |ev| {
                    if let Some(on_click) = on_click {
                        on_click.run(ev);
                    }
                }
            >
                {content}
            </div>
        }
        .into_any(),
        // default "button"
        _ => view! {
            <button
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
                on:click=move |ev| {
                    if let Some(on_click) = on_click {
                        on_click.run(ev);
                    }
                }
            >
                {content}
            </button>
        }
        .into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn delete_renders_default_button() {
        let html = view! { <Delete></Delete> }.to_html();
        assert!(
            html.contains(r#"class="delete""#),
            "expected base 'delete' class, got: {}",
            html
        );
        assert!(
            html.contains("<button"),
            "expected default button tag, got: {}",
            html
        );
    }

    #[test]
    fn delete_custom_tag_and_classes() {
        let html = view! { <Delete tag="a" classes="is-large"></Delete> }.to_html();
        assert!(
            html.contains(r#"class="delete is-large""#),
            "expected combined classes, got: {}",
            html
        );
        assert!(html.contains("<a"), "expected anchor tag, got: {}", html);
    }

    #[test]
    fn delete_renders_children() {
        let html = view! { <Delete>"X"</Delete> }.to_html();
        assert!(
            html.contains(">X<"),
            "expected child content, got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn delete_renders_test_id() {
        let html = view! {
            <Delete test_attr=TestAttr::test_id("delete-test")></Delete>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="delete-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn delete_no_test_attr_when_not_provided() {
        let html = view! { <Delete></Delete> }.to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn delete_accepts_custom_test_attr_key() {
        let html = view! {
            <Delete test_attr=TestAttr::new("data-cy", "delete-cy")></Delete>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="delete-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
