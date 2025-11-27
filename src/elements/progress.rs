use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, Get, Signal};
use leptos::tachys::view::any_view::IntoAny;
use leptos::{component, view, IntoView};

use crate::util::TestAttr;

/// A native HTML progress bar.
///
/// https://bulma.io/documentation/elements/progress/
#[component]
pub fn Progress(
    #[prop(optional, into)] classes: Signal<String>,
    /// The maximum amount of progress; the 100% value.
    #[prop(default = 1.0.into(), into)]
    max: Signal<f32>,
    /// The amount of progress which has been made.
    /// Use -1.0 for an indeterminate progress bar.
    #[prop(default = 0.0.into(), into)]
    value: Signal<f32>,
    /// Optional test attribute (renders as data-* attribute) on the <progress> element.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (for example, `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let class = move || {
        let extras = classes.get();
        if extras.trim().is_empty() {
            "progress".to_string()
        } else {
            format!("progress {}", extras.trim())
        }
    };

    let max_value = move || max.get();
    let current_value = move || value.get();
    let is_indeterminate = move || current_value() == -1.0;

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    move || {
        if is_indeterminate() {
            view! {
                <progress
                    class=class
                    max=move || max_value()
                    attr:data-testid=move || data_testid.clone()
                    attr:data-cy=move || data_cy.clone()
                />
            }
            .into_any()
        } else {
            view! {
                <progress
                    class=class
                    max=move || max_value()
                    value=move || current_value()
                    attr:data-testid=move || data_testid.clone()
                    attr:data-cy=move || data_cy.clone()
                >
                    {move || format!("{:.0}%", current_value())}
                </progress>
            }
            .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn progress_renders_with_defaults() {
        let html = view! { <Progress /> }.to_html();
        assert!(
            html.contains(r#"class="progress""#),
            "expected base progress class, got: {html}"
        );
        assert!(
            html.contains(r#"max="1""#),
            "expected default max=1, got: {html}"
        );
    }

    #[test]
    fn progress_renders_with_value() {
        let html = view! { <Progress max=100.0 value=50.0 /> }.to_html();
        assert!(
            html.contains(r#"max="100""#),
            "expected max=100, got: {html}"
        );
        assert!(
            html.contains(r#"value="50""#),
            "expected value=50, got: {html}"
        );
        assert!(
            html.contains("50%"),
            "expected percentage text, got: {html}"
        );
    }

    #[test]
    fn progress_renders_indeterminate() {
        let html = view! { <Progress max=100.0 value=-1.0 /> }.to_html();
        assert!(
            html.contains(r#"class="progress""#),
            "expected progress class, got: {html}"
        );
        assert!(
            !html.contains(r#"value="#),
            "expected no value attribute for indeterminate, got: {html}"
        );
    }

    #[test]
    fn progress_appends_custom_classes() {
        let html = view! { <Progress classes="is-primary is-large" /> }.to_html();
        assert!(
            html.contains(r#"class="progress is-primary is-large""#),
            "expected additional classes, got: {html}"
        );
    }

    #[test]
    fn progress_with_zero_value() {
        let html = view! { <Progress max=100.0 value=0.0 /> }.to_html();
        assert!(
            html.contains(r#"value="0""#),
            "expected value=0, got: {html}"
        );
        assert!(html.contains("0%"), "expected 0% text, got: {html}");
    }

    #[test]
    fn progress_with_max_value() {
        let html = view! { <Progress max=100.0 value=100.0 /> }.to_html();
        assert!(
            html.contains(r#"value="100""#),
            "expected value=100, got: {html}"
        );
        assert!(html.contains("100%"), "expected 100% text, got: {html}");
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
    fn progress_renders_test_id() {
        let html = view! {
            <Progress test_attr=TestAttr::test_id("progress-test") />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="progress-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn progress_no_test_attr_when_not_provided() {
        let html = view! {
            <Progress />
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn progress_accepts_custom_test_attr_key() {
        let html = view! {
            <Progress test_attr=TestAttr::new("data-cy", "progress-cy") />
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="progress-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
