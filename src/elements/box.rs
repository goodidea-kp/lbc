/*!
Leptos version of Bulma Box element.

Bulma docs: https://bulma.io/documentation/elements/box/
*/

use leptos::prelude::{
    Children, ClassAttribute, ElementChild, Get, GetUntracked, IntoView, Signal, component, view,
};

/// A white box to contain other elements.
#[component]
pub fn Box(
    /// Additional CSS classes to append to the base "box" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// Child content to render inside the box
    children: Children,
) -> impl IntoView {
    // Build class attribute: "box [extra classes]"
    let mut class_attr = String::from("box");

    if let Some(extra) = classes {
        let extra_val = extra.get_untracked();
        if !extra_val.trim().is_empty() {
            class_attr.push(' ');
            class_attr.push_str(extra_val.trim());
        }
    }

    view! { <div class=class_attr>{children()}</div> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn box_renders_default() {
        let html = view! { <Box>"X"</Box> }.to_html();
        assert!(
            html.contains(r#"class="box""#),
            "expected base 'box' class, got: {}",
            html
        );
        assert!(html.contains('X'));
    }

    #[test]
    fn box_with_extra_classes() {
        let html = view! { <Box classes="has-shadow mt-2">"Y"</Box> }.to_html();
        assert!(
            html.contains(r#"class="box has-shadow mt-2""#),
            "expected combined classes, got: {}",
            html
        );
        assert!(html.contains('Y'));
    }
}
