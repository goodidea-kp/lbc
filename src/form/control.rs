use leptos::prelude::IntoAny;
use leptos::prelude::*;

/// A container with which you can wrap form controls (Bulma "control").
///
/// https://bulma.io/documentation/form/general/
///
/// Props:
/// - `classes`: extra CSS classes appended to "control"
/// - `tag`: optional custom HTML tag name (defaults to "div")
/// - `expanded`: when true, adds "is-expanded"
/// - `children`: inner content (typically inputs/buttons)
#[component]
pub fn Control(
    /// Additional CSS classes to append to Bulma's "control".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// The HTML tag to use for this component. Defaults to "div".
    #[prop(optional, into)]
    tag: Option<Signal<String>>,

    /// A modifier to have the controlled element fill up the remaining space.
    #[prop(optional, into)]
    expanded: Signal<bool>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

    /// Child nodes rendered inside the control.
    children: Children,
) -> impl IntoView {
    let class = move || {
        let mut parts = vec!["control".to_string()];

        let extra = classes.get();
        if !extra.trim().is_empty() {
            parts.push(extra);
        }

        if expanded.get() {
            parts.push("is-expanded".to_string());
        }

        parts.join(" ")
    };

    let tag_name = move || {
        tag.as_ref()
            .map(|t| t.get())
            .unwrap_or_else(|| "div".to_string())
    };

    {
        let current = tag_name();
        match current.as_str() {
            "article" => {
                view! { <article class=class data-testid=test_id.clone()>{children()}</article> }
                    .into_any()
            }
            "label" => {
                view! { <label class=class data-testid=test_id.clone()>{children()}</label> }
                    .into_any()
            }
            "p" => {
                view! { <p class=class data-testid=test_id.clone()>{children()}</p> }.into_any()
            }
            _ => {
                view! { <div class=class data-testid=test_id>{children()}</div> }.into_any()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn control_renders_default_div_with_base_class() {
        let html = view! { <Control>"X"</Control> }.to_html();
        assert!(
            html.contains(r#"class="control""#),
            "expected base 'control' class, got: {}",
            html
        );
        assert!(
            html.contains("<div"),
            "expected default div tag, got: {}",
            html
        );
        assert!(html.contains('X'));
    }

    #[test]
    fn control_with_custom_tag_and_classes() {
        let html = view! { <Control tag="article" classes="my extra">"Y"</Control> }.to_html();
        assert!(
            html.contains(r#"class="control my extra""#),
            "expected combined classes, got: {}",
            html
        );
        assert!(
            html.contains("<article"),
            "expected article tag, got: {}",
            html
        );
        assert!(html.contains('Y'));
    }

    #[test]
    fn control_expanded_adds_modifier_class() {
        let html = view! { <Control expanded=true>"Z"</Control> }.to_html();
        assert!(
            html.contains("is-expanded"),
            "expected 'is-expanded' class when expanded=true, got: {}",
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
    fn control_renders_test_id() {
        let html = view! {
            <Control test_id="control-test">"Content"</Control>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="control-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn control_no_test_id_when_not_provided() {
        let html = view! {
            <Control>"Content"</Control>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute; got: {}",
            html
        );
    }
}
