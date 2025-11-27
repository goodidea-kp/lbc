use leptos::children::Children;
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, Get, Signal};
use leptos::{component, view, IntoView};

use crate::util::TestAttr;

/// Bold notification blocks, to alert your users of something.
///
/// https://bulma.io/documentation/elements/notification/
#[component]
pub fn Notification(
    #[prop(optional, into)] classes: Signal<String>,
    /// Optional test attribute (renders as data-* attribute) on the root <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (for example, `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
    children: Children,
) -> impl IntoView {
    let class = move || {
        let extras = classes.get();
        if extras.trim().is_empty() {
            "notification".to_string()
        } else {
            format!("notification {}", extras.trim())
        }
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
    fn notification_renders_children() {
        let html = view! { <Notification>{"Heads up!"}</Notification> }.to_html();
        assert!(
            html.contains(r#"class="notification""#),
            "expected base notification class, got: {html}"
        );
        assert!(html.contains("Heads up!"), "expected children to render");
    }

    #[test]
    fn notification_appends_custom_classes() {
        let html =
            view! { <Notification classes="is-link is-light">{"Link notice"}</Notification> }
                .to_html();
        assert!(
            html.contains(r#"class="notification is-link is-light""#),
            "expected additional classes, got: {html}"
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
    fn notification_renders_test_id() {
        let html = view! {
            <Notification test_attr=TestAttr::test_id("notification-test")>{"Content"}</Notification>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="notification-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn notification_no_test_attr_when_not_provided() {
        let html = view! {
            <Notification>{"Content"}</Notification>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn notification_accepts_custom_test_attr_key() {
        let html = view! {
            <Notification test_attr=TestAttr::new("data-cy", "notification-cy")>{"Content"}</Notification>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="notification-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
