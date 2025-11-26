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
    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
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

    view! { <div class=class_attr data-testid=test_id>{children()}</div> }
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

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn block_renders_test_id() {
        let html = view! {
            <Block test_id="block-test">"Content"</Block>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="block-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn block_no_test_id_when_not_provided() {
        let html = view! {
            <Block>"Content"</Block>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute; got: {}",
            html
        );
    }
}
