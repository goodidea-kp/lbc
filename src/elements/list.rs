/*!
Simple wrapper around HTML list elements.

Bulma doesn't define a specific list component; this provides a thin
Leptos component to render unordered or ordered lists with optional classes.
*/

use leptos::prelude::{
    AnyView, Children, ClassAttribute, CustomAttribute, ElementChild, Get, GetUntracked, IntoAny,
    Signal, component, view,
};

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

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

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

    match (tag_name.as_str(), class_value.is_empty()) {
        ("ol", true) => view! { <ol data-testid=test_id>{children()}</ol> }.into_any(),
        ("ol", false) => {
            view! { <ol class=class_value.clone() data-testid=test_id>{children()}</ol> }.into_any()
        }
        (_, true) => view! { <ul data-testid=test_id>{children()}</ul> }.into_any(),
        (_, false) => {
            view! { <ul class=class_value.clone() data-testid=test_id>{children()}</ul> }.into_any()
        }
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
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn list_renders_test_id() {
        let html = view! {
            <List test_id="list-test"><li>"A"</li></List>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="list-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn list_no_test_id_when_not_provided() {
        let html = view! {
            <List><li>"A"</li></List>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute; got: {}",
            html
        );
    }
}
