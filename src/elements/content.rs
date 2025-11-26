/*!
Leptos version of Bulma Content element.

Bulma docs: https://bulma.io/documentation/elements/content/
*/

use leptos::prelude::{
    AnyView, Children, ClassAttribute, ElementChild, Get, GetUntracked, IntoAny, Signal, component,
    view,
};

/// A single component to wrap WYSIWYG generated content, where only HTML tags are available.
#[component]
pub fn Content(
    /// Additional CSS classes to append to the base "content" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (div, article, section, nav, p, span)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,
    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
    /// Child content to render inside the content block
    children: Children,
) -> AnyView {
    // Build class attribute: "content [extra classes]"
    let mut class_attr = String::from("content");

    if let Some(extra) = classes {
        let extra_val = extra.get_untracked();
        if !extra_val.trim().is_empty() {
            class_attr.push(' ');
            class_attr.push_str(extra_val.trim());
        }
    }

    let tag_name = tag
        .as_ref()
        .map(|t| t.get().to_lowercase())
        .unwrap_or_else(|| "div".to_string());

    match tag_name.as_str() {
        "article" => {
            view! { <article class=class_attr.clone() data-testid=test_id.clone()>{children()}</article> }
                .into_any()
        }
        "section" => {
            view! { <section class=class_attr.clone() data-testid=test_id.clone()>{children()}</section> }
                .into_any()
        }
        "nav" => {
            view! { <nav class=class_attr.clone() data-testid=test_id.clone()>{children()}</nav> }
                .into_any()
        }
        "p" => {
            view! { <p class=class_attr.clone() data-testid=test_id.clone()>{children()}</p> }
                .into_any()
        }
        "span" => {
            view! { <span class=class_attr.clone() data-testid=test_id.clone()>{children()}</span> }
                .into_any()
        }
        _ => {
            view! { <div class=class_attr.clone() data-testid=test_id>{children()}</div> }.into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn content_renders_default_div() {
        let html = view! { <Content><p>"Hello"</p></Content> }.to_html();
        assert!(
            html.contains(r#"class="content""#),
            "expected base 'content' class, got: {}",
            html
        );
        assert!(
            html.contains("<div"),
            "expected default div tag, got: {}",
            html
        );
        assert!(html.contains(">Hello<"), "expected child html present");
    }

    #[test]
    fn content_with_custom_tag_and_classes() {
        let html =
            view! { <Content tag="article" classes="is-small"><p>"X"</p></Content> }.to_html();
        assert!(
            html.contains(r#"class="content is-small""#),
            "expected combined classes, got: {}",
            html
        );
        assert!(
            html.contains("<article"),
            "expected article tag, got: {}",
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
    fn content_renders_test_id() {
        let html = view! {
            <Content test_id="content-test"><p>"Content"</p></Content>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="content-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn content_no_test_id_when_not_provided() {
        let html = view! {
            <Content><p>"Content"</p></Content>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute; got: {}",
            html
        );
    }
}
