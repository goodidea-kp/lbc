/*!
Simple wrapper around HTML list elements.

Bulma doesn't define a specific list component; this provides a thin
Leptos component to render unordered or ordered lists with optional classes.
*/

use leptos::prelude::{
    AnyView, Children, ClassAttribute, ElementChild, Get, GetUntracked, IntoAny, Signal, component,
    view,
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
    /// List items (typically <li>...</li>)
    children: Children,
) -> AnyView {
    let class_value = classes
        .as_ref()
        .map(|s| s.get_untracked().trim().to_string())
        .unwrap_or_default();

    let tag_name = tag
        .as_ref()
        .map(|t| t.get().to_lowercase())
        .unwrap_or_else(|| "ul".to_string());

    match (tag_name.as_str(), class_value.is_empty()) {
        ("ol", true) => view! { <ol>{children()}</ol> }.into_any(),
        ("ol", false) => view! { <ol class=class_value.clone()>{children()}</ol> }.into_any(),
        (_, true) => view! { <ul>{children()}</ul> }.into_any(),
        (_, false) => view! { <ul class=class_value.clone()>{children()}</ul> }.into_any(),
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
