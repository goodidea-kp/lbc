/*!
Leptos version of Bulma Media component.

- Media: renders a Bulma "media" element with optional custom tag and classes
- MediaLeft: left-aligned area within a media object
- MediaRight: right-aligned area within a media object
- MediaContent: central body/content area within a media object

Follows existing crate patterns:
- optional props via #[prop(optional)]
- classes as Option<Signal<String>>
- dynamic tag selection like Tile/Level components
*/

use leptos::prelude::{
    AnyView, Children, ClassAttribute, ElementChild, Get, GetUntracked, IntoAny, Signal, component, view,
};

/// A UI element for repeatable and nestable content.
/// https://bulma.io/documentation/layout/media-object/
#[component]
pub fn Media(
    #[prop(optional, into)] classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (div, article, section, nav, p, span)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,
    children: Children,
) -> AnyView {
    // Build class attribute: "media [extra classes]"
    let mut class_attr = String::from("media");

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
        "article" => view! { <article class=class_attr.clone()>{children()}</article> }.into_any(),
        "section" => view! { <section class=class_attr.clone()>{children()}</section> }.into_any(),
        "nav" => view! { <nav class=class_attr.clone()>{children()}</nav> }.into_any(),
        "p" => view! { <p class=class_attr.clone()>{children()}</p> }.into_any(),
        "span" => view! { <span class=class_attr.clone()>{children()}</span> }.into_any(),
        _ => view! { <div class=class_attr.clone()>{children()}</div> }.into_any(),
    }
}

/// Elements to be grouped to the left of the media container.
#[component]
pub fn MediaLeft(
    #[prop(optional, into)] classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (div, article, section, nav, p, span)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,
    children: Children,
) -> AnyView {
    let mut class_attr = String::from("media-left");

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
        "article" => view! { <article class=class_attr.clone()>{children()}</article> }.into_any(),
        "section" => view! { <section class=class_attr.clone()>{children()}</section> }.into_any(),
        "nav" => view! { <nav class=class_attr.clone()>{children()}</nav> }.into_any(),
        "p" => view! { <p class=class_attr.clone()>{children()}</p> }.into_any(),
        "span" => view! { <span class=class_attr.clone()>{children()}</span> }.into_any(),
        _ => view! { <div class=class_attr.clone()>{children()}</div> }.into_any(),
    }
}

/// Elements to be grouped to the right of the media container.
#[component]
pub fn MediaRight(
    #[prop(optional, into)] classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (div, article, section, nav, p, span)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,
    children: Children,
) -> AnyView {
    let mut class_attr = String::from("media-right");

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
        "article" => view! { <article class=class_attr.clone()>{children()}</article> }.into_any(),
        "section" => view! { <section class=class_attr.clone()>{children()}</section> }.into_any(),
        "nav" => view! { <nav class=class_attr.clone()>{children()}</nav> }.into_any(),
        "p" => view! { <p class=class_attr.clone()>{children()}</p> }.into_any(),
        "span" => view! { <span class=class_attr.clone()>{children()}</span> }.into_any(),
        _ => view! { <div class=class_attr.clone()>{children()}</div> }.into_any(),
    }
}

/// Elements to be grouped as the center body of the media container.
#[component]
pub fn MediaContent(
    #[prop(optional, into)] classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (div, article, section, nav, p, span)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,
    children: Children,
) -> AnyView {
    let mut class_attr = String::from("media-content");

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
        "article" => view! { <article class=class_attr.clone()>{children()}</article> }.into_any(),
        "section" => view! { <section class=class_attr.clone()>{children()}</section> }.into_any(),
        "nav" => view! { <nav class=class_attr.clone()>{children()}</nav> }.into_any(),
        "p" => view! { <p class=class_attr.clone()>{children()}</p> }.into_any(),
        "span" => view! { <span class=class_attr.clone()>{children()}</span> }.into_any(),
        _ => view! { <div class=class_attr.clone()>{children()}</div> }.into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn media_renders_default() {
        let html = view! { <Media>"X"</Media> }.to_html();
        assert!(
            html.contains(r#"class="media""#),
            "expected base 'media' class, got: {}",
            html
        );
        assert!(
            html.contains("<div"),
            "expected default div tag, got: {}",
            html
        );
    }

    #[test]
    fn media_with_custom_tag_and_classes() {
        let html = view! { <Media tag="article" classes="custom-class">"X"</Media> }.to_html();
        assert!(
            html.contains(r#"class="media custom-class""#),
            "expected combined classes, got: {}",
            html
        );
        assert!(
            html.contains("<article"),
            "expected article tag, got: {}",
            html
        );
    }

    #[test]
    fn media_structure_left_content_right() {
        let html = view! {
            <Media>
                <MediaLeft><span>"L"</span></MediaLeft>
                <MediaContent><p>"C"</p></MediaContent>
                <MediaRight><span>"R"</span></MediaRight>
            </Media>
        }
        .to_html();
        assert!(
            html.contains(r#"class="media-left""#),
            "expected media-left"
        );
        assert!(
            html.contains(r#"class="media-content""#),
            "expected media-content"
        );
        assert!(
            html.contains(r#"class="media-right""#),
            "expected media-right"
        );
    }
}
