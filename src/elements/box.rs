/*!
Leptos version of Bulma Box element.

Bulma docs: https://bulma.io/documentation/elements/box/
*/

use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, GetUntracked, IntoView, Signal,
    component, view,
};

use crate::util::TestAttr;

/// A white box to contain other elements.
#[component]
pub fn Box(
    /// Additional CSS classes to append to the base "box" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute) on the root <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

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

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn box_renders_test_id() {
        let html = view! {
            <Box test_attr=TestAttr::test_id("box-test")>"Content"</Box>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="box-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn box_no_test_id_when_not_provided() {
        let html = view! {
            <Box>"Content"</Box>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn box_accepts_custom_test_attr_key() {
        let html = view! {
            <Box test_attr=TestAttr::new("data-cy", "box-cy")>"Content"</Box>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="box-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
