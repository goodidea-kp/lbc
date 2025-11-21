use leptos::children::Children;
use leptos::prelude::{ClassAttribute, ElementChild, Get, Signal};
use leptos::{IntoView, component, view};

/// Bold notification blocks, to alert your users of something.
///
/// https://bulma.io/documentation/elements/notification/
#[component]
pub fn Notification(
    #[prop(optional, into)] classes: Signal<String>,
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
        <div class=class>
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
