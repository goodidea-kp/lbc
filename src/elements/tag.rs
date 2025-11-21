/*!
Leptos version of the Bulma Tag element.

Bulma docs: https://bulma.io/documentation/elements/tag/
*/

use leptos::children::Children;
use leptos::prelude::{ClassAttribute, ElementChild, Get, Signal};
use leptos::{IntoView, component, view};

use crate::util::Size;

/// Available color variants for a Bulma tag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagColor {
    Black,
    Danger,
    Dark,
    Info,
    Light,
    Link,
    Primary,
    Success,
    Warning,
    White,
}
impl TagColor {
    /// Returns the Bulma CSS class for this `TagColor`.
    fn bulma(self) -> &'static str {
        match self {
            TagColor::Black => "is-black",
            TagColor::Danger => "is-danger",
            TagColor::Dark => "is-dark",
            TagColor::Info => "is-info",
            TagColor::Light => "is-light",
            TagColor::Link => "is-link",
            TagColor::Primary => "is-primary",
            TagColor::Success => "is-success",
            TagColor::Warning => "is-warning",
            TagColor::White => "is-white",
        }
    }
}

/// Label-like UI element useful for tagging and categorization.
///
/// https://bulma.io/documentation/elements/tag/
#[component]
pub fn Tag(
    /// Optional color of the tag.
    #[prop(optional)]
    color: Option<TagColor>,
    /// Optional size of the tag.
    #[prop(optional)]
    size: Option<Size>,
    /// Render a lighter color variant when true.
    #[prop(optional)]
    light: bool,
    /// Render fully rounded corners when true.
    #[prop(optional)]
    rounded: bool,
    /// Additional CSS classes to append to the base "tag" class.
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// Child content to render inside the tag.
    children: Children,
) -> impl IntoView {
    let class = move || {
        let mut class_parts: Vec<&str> = vec!["tag"];
        if let Some(color_value) = color {
            class_parts.push(color_value.bulma());
        }
        if let Some(size_value) = size {
            let size_class = size_value.bulma();
            if !size_class.is_empty() {
                class_parts.push(size_class);
            }
        }
        if light {
            class_parts.push("is-light");
        }
        if rounded {
            class_parts.push("is-rounded");
        }
        if let Some(class_signal) = &classes {
            let extra_classes = class_signal.get();
            if !extra_classes.is_empty() {
                return format!("{} {}", class_parts.join(" "), extra_classes);
            }
        }
        class_parts.join(" ")
    };
    view! { <span class=class>{children()}</span> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn tag_renders_basic() {
        let html = view! { <Tag>"Hi"</Tag> }.to_html();
        assert!(html.contains(r#"class="tag""#), "expected base tag class");
        assert!(html.contains(">Hi<"), "expected tag content");
    }

    #[test]
    fn tag_color_and_rounded() {
        let html = view! { <Tag color=TagColor::Success rounded=true>"OK"</Tag> }.to_html();
        assert!(
            html.contains(r#"class="tag is-success is-rounded""#),
            "expected color and rounded classes"
        );
    }
}
