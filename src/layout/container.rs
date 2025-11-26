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

/// A simple responsive container to center and constrain your content.
///
/// https://bulma.io/documentation/layout/container/
#[component]
pub fn Container(
    #[prop(optional)] fluid: bool,
    #[prop(optional, into)] classes: Option<Signal<String>>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

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
    view! { <div class=class data-testid=test_id>{children()}</div> }
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
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn container_renders_test_id() {
        let html = view! {
            <Container fluid=true test_id="container-test">
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
    fn container_no_test_id_when_not_provided() {
        let html = view! {
            <Container fluid=true>
                "X"
            </Container>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on Container when not provided; got: {}",
            html
        );
    }
}
