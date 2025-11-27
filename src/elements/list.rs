/*!
Simple wrapper around HTML list elements.

Bulma doesn't define a specific list component; this provides a thin
Leptos component to render unordered or ordered lists with optional classes.
*/

use leptos::prelude::{
    AnyView, Children, ClassAttribute, CustomAttribute, ElementChild, Get, GetUntracked, IntoAny,
    Signal, component, view,
};

use crate::util::TestAttr;

/// A simple list component that renders an unordered (<ul>) or ordered (<ol>) list.
///
/// Defaults to <ul>. Pass tag="ol" to render an ordered list.
#[component]
pub fn List(
    /// Additional CSS classes to apply to the list element
    #[prop(optional, into)]
    classes: Option<Signal<String>>,

    /// The HTML tag to use for this component: "ul" (default) or "ol"
    #[prop(optional, into)]
    tag: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute) on the rendered list.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (for example, `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    /// List items (typically <li>...</li>)
    children: Children,
) -> AnyView {
    let class_value = classes
        .as_ref()
        .map(|signal| signal.get_untracked().trim().to_string())
        .unwrap_or_default();

    let tag_name = tag
        .as_ref()
        .map(|signal| signal.get().to_lowercase())
        .unwrap_or_else(|| "ul".to_string());

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    match (tag_name.as_str(), class_value.is_empty()) {
        ("ol", true) => view! {
            <ol
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </ol>
        }
        .into_any(),
        ("ol", false) => view! {
            <ol
                class=class_value.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </ol>
        }
        .into_any(),
        (_, true) => view! {
            <ul
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </ul>
        }
        .into_any(),
        (_, false) => view! {
            <ul
                class=class_value.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </ul>
        }
        .into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn list_renders_ul_by_default() {
        let html = view! { <List><li>"A"</li></List> }.to_html();
        assert!(
            html.contains("<ul"),
            "expected default ul tag, got: {}",
            html
        );
        assert!(
            html.contains(">A<"),
            "expected list item content, got: {}",
            html
        );
    }

    #[test]
    fn list_can_render_ol_with_classes() {
        let html = view! { <List tag="ol" classes="is-lower-alpha"><li>"a"</li></List> }.to_html();
        assert!(html.contains("<ol"), "expected ol tag, got: {}", html);
        assert!(
            html.contains(r#"class="is-lower-alpha""#),
            "expected custom class, got: {}",
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
    fn list_renders_test_id() {
        let html = view! {
            <List test_attr=TestAttr::test_id("list-test")><li>"A"</li></List>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="list-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn list_no_test_attr_when_not_provided() {
        let html = view! {
            <List><li>"A"</li></List>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn list_accepts_custom_test_attr_key() {
        let html = view! {
            <List test_attr=TestAttr::new("data-cy", "list-cy")><li>"A"</li></List>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="list-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
