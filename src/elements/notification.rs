use leptos::children::Children;
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, Get, Signal};
use leptos::{component, view, IntoView};

/// Bold notification blocks, to alert your users of something.
///
/// https://bulma.io/documentation/elements/notification/
#[component]
pub fn Notification(
    #[prop(optional, into)] classes: Signal<String>,
    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
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

    view! {
        <div class=class data-testid=test_id>
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
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn notification_renders_test_id() {
        let html = view! {
            <Notification test_id="notification-test">{"Content"}</Notification>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="notification-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn notification_no_test_id_when_not_provided() {
        let html = view! {
            <Notification>{"Content"}</Notification>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute; got: {}",
            html
        );
    }
}
