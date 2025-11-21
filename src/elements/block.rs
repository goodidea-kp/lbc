/*!
Leptos version of Bulma Block element.

Bulma docs: https://bulma.io/documentation/elements/block/
*/

use leptos::prelude::{
    Children, ClassAttribute, ElementChild, GetUntracked, IntoView, Signal, component, view,
};

/// Bulma’s most basic spacer block
#[component]
pub fn Block(
    /// Additional CSS classes to append to the base "block" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// Child content to render inside the block
    children: Children,
) -> impl IntoView {
    // Build class attribute: "block [extra classes]"
    let mut class_attr = String::from("block");

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
    fn block_renders_default() {
        let html = view! { <Block>"X"</Block> }.to_html();
        assert!(
            html.contains(r#"class="block""#),
            "expected base 'block' class, got: {}",
            html
        );
        assert!(html.contains('X'));
    }

    #[test]
    fn block_with_extra_classes() {
        let html = view! { <Block classes="my cls">"Y"</Block> }.to_html();
        assert!(
            html.contains(r#"class="block my cls""#),
            "expected combined classes, got: {}",
            html
        );
        assert!(html.contains('Y'));
    }
}
