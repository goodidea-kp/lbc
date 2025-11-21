use leptos::children::Children;
use leptos::prelude::{ClassAttribute, ElementChild, Get, Signal};
use leptos::{IntoView, component, view};

use crate::util::Size;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagColor {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Dark,
    Light,
    Black,
    White,
}
impl TagColor {
    fn bulma(self) -> &'static str {
        match self {
            TagColor::Primary => "is-primary",
            TagColor::Link => "is-link",
            TagColor::Info => "is-info",
            TagColor::Success => "is-success",
            TagColor::Warning => "is-warning",
            TagColor::Danger => "is-danger",
            TagColor::Dark => "is-dark",
            TagColor::Light => "is-light",
            TagColor::Black => "is-black",
            TagColor::White => "is-white",
        }
    }
}

#[component]
pub fn Tag(
    #[prop(optional)] color: Option<TagColor>,
    #[prop(optional)] size: Option<Size>,
    #[prop(optional)] light: bool,
    #[prop(optional)] rounded: bool,
    #[prop(optional, into)] classes: Option<Signal<String>>,
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
