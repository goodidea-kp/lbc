/*!
Leptos version of Bulma Footer component.

- Footer: wraps content in a Bulma "footer" element

Follows existing crate patterns:
- optional props via #[prop(optional)] / #[prop(optional, into)]
- classes as Option<Signal<String>>
*/

use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, GetUntracked, IntoView, Signal,
    component, view,
};

use crate::util::TestAttr;

/// A simple responsive footer which can include anything.
///
/// https://bulma.io/documentation/layout/footer/
#[component]
pub fn Footer(
    #[prop(optional, into)] classes: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute)
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    children: Children,
) -> impl IntoView {
    // Build class attribute: "footer [extra classes]"
    let mut class_attr = String::from("footer");

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
        <footer
            class=class_attr
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {children()}
        </footer>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn footer_renders_default() {
        let html = view! { <Footer>"X"</Footer> }.to_html();
        assert!(
            html.contains(r#"class="footer""#),
            "expected base 'footer' class, got: {}",
            html
        );
        assert!(html.contains('X'));
    }

    #[test]
    fn footer_with_extra_classes() {
        let html =
            view! { <Footer classes="has-background-dark has-text-white">"Y"</Footer> }.to_html();
        assert!(
            html.contains(r#"class="footer has-background-dark has-text-white""#),
            "expected combined classes, got: {}",
            html
        );
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
    fn footer_renders_test_attr_as_data_testid() {
        let html = view! {
            <Footer classes="has-background-dark" test_attr=TestAttr::test_id("footer-test")>
                "X"
            </Footer>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="footer-test""#),
            "expected data-testid attribute on Footer; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn footer_no_test_attr_when_not_provided() {
        let html = view! {
            <Footer classes="has-background-dark">
                "X"
            </Footer>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute on Footer when not provided; got: {}",
            html
        );
    }
}
