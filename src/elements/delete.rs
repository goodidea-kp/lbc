/*!
Leptos version of Bulma Delete element.

Bulma docs: https://bulma.io/documentation/elements/delete/
*/

use leptos::ev::MouseEvent;
use leptos::prelude::{
    AnyView, Children, ClassAttribute, ElementChild, Get, IntoAny, OnAttribute, Signal, component,
    view,
};
use std::sync::Arc;

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
    on_click: Option<Arc<dyn Fn(MouseEvent) + Send + Sync>>,
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
        .map(|t| t.get().to_lowercase())
        .unwrap_or_else(|| "button".to_string());

    // Render children only if provided; otherwise render nothing.
    let content = match children {
        Some(c) => c(),
        None => view! {}.into_any(),
    };

    let render_click = |handler: Option<Arc<dyn Fn(MouseEvent) + Send + Sync>>| {
        move |event: MouseEvent| {
            if let Some(cb) = handler.as_ref() {
                (cb)(event);
            }
        }
    };

    match tag_name.as_str() {
        "a" => {
            let handler = on_click.clone();
            view! { <a class=class_attr.clone() on:click=render_click(handler)>{content}</a> }
                .into_any()
        }
        "span" => {
            let handler = on_click.clone();
            view! { <span class=class_attr.clone() on:click=render_click(handler)>{content}</span> }
                .into_any()
        }
        "div" => {
            let handler = on_click.clone();
            view! { <div class=class_attr.clone() on:click=render_click(handler)>{content}</div> }
                .into_any()
        }
        // default "button"
        _ => {
            let handler = on_click;
            view! { <button class=class_attr.clone() on:click=render_click(handler)>{content}</button> }.into_any()
        }
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
