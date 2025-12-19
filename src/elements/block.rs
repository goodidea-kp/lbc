/*!
Leptos version of Bulma Block element.

Bulma docs: https://bulma.io/documentation/elements/block/
*/

use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, GetUntracked, IntoView, Signal,
    component, view,
};

use crate::util::TestAttr;

/// Bulma’s most basic spacer block
#[component]
pub fn Block(
    /// Additional CSS classes to append to the base "block" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute) on the root <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

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

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <div
            class=class_attr
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {children()}
        </div>
    }
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

    #[test]
    fn block_renders_test_id() {
        let html = view! {
            <Block test_attr=TestAttr::test_id("block-test")>"Content"</Block>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="block-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[test]
    fn block_no_test_id_when_not_provided() {
        let html = view! {
            <Block>"Content"</Block>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[test]
    fn block_accepts_custom_test_attr_key() {
        let html = view! {
            <Block test_attr=TestAttr::new("data-cy", "block-cy")>"Content"</Block>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="block-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    // No wasm-specific tests needed for now.
}
