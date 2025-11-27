/*!
Leptos version of Bulma Container layout.

- Container: a simple responsive fixed-width container

Follows existing crate patterns:
- optional props via #[prop(optional)] / #[prop(optional, into)]
- classes as Option<Signal<String>>
*/

use leptos::children::Children;
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, Get, Signal};
use leptos::{IntoView, component, view};

use crate::util::TestAttr;

/// A simple responsive container to center and constrain your content.
///
/// https://bulma.io/documentation/layout/container/
#[component]
pub fn Container(
    #[prop(optional)] fluid: bool,
    #[prop(optional, into)] classes: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute)
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    children: Children,
) -> impl IntoView {
    let class = move || {
        let mut class_parts: Vec<&str> = vec!["container"];
        if fluid {
            class_parts.push("is-fluid");
        }
        if let Some(class_signal) = &classes {
            let extra_classes = class_signal.get();
            if !extra_classes.is_empty() {
                return format!("{} {}", class_parts.join(" "), extra_classes);
            }
        }
        class_parts.join(" ")
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <div
            class=class
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
    fn container_fluid() {
        let html = view! { <Container fluid=true>"X"</Container> }.to_html();
        assert!(
            html.contains("container") && html.contains("is-fluid"),
            "expected container fluid class, got: {}",
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
    fn container_renders_test_attr_as_data_testid() {
        let html = view! {
            <Container fluid=true test_attr=TestAttr::test_id("container-test")>
                "X"
            </Container>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="container-test""#),
            "expected data-testid attribute on Container; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn container_no_test_attr_when_not_provided() {
        let html = view! {
            <Container fluid=true>
                "X"
            </Container>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute on Container when not provided; got: {}",
            html
        );
    }
}
