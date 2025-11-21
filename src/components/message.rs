use leptos::prelude::{component, view, Children, ClassAttribute, ElementChild, Get, IntoView, OnAttribute, Set, Signal, StyleAttribute, AriaAttributes};
use std::rc::Rc;

fn base_class(extra: &str) -> String {
    if extra.trim().is_empty() {
        "message".to_string()
    } else {
        format!("message {}", extra)
    }
}

/// Colored message blocks, to emphasize part of your page.
/// https://bulma.io/documentation/components/message/
#[component]
pub fn Message(
    /// Extra classes to apply to the Bulma "message" container (e.g., is-primary, is-warning).
    #[prop(optional, into)]
    classes: Signal<String>,

    /// When true, renders a close button in the top-right that hides the entire message when clicked.
    #[prop(optional, into)]
    closable: Signal<bool>,

    /// Optional close callback invoked when the close button is clicked.
    #[prop(optional)]
    on_close: Option<Rc<dyn Fn()>>,

    /// Child content of the message (usually MessageHeader and MessageBody).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class(&classes.get())
    };

    let (is_closed, set_is_closed) = leptos::prelude::signal(false);
    let on_close_click = {
        let on_close = on_close.clone();
        move |_| {
            if let Some(cb) = &on_close {
                cb();
            } else {
                set_is_closed.set(true);
            }
        }
    };

    view! {
        <article
            class=class
            style=move || {
                let mut parts: Vec<&str> = Vec::new();
                if closable.get() {
                    parts.push("position: relative;");
                }
                if is_closed.get() {
                    parts.push("display: none;");
                }
                parts.join(" ")
            }
        >
            <button
                class="delete is-small"
                aria-label="delete"
                style=move || if closable.get() && !is_closed.get() {
                    "position:absolute; right:0.5rem; top:0.5rem; z-index: 10;"
                } else {
                    "display: none;"
                }
                on:click=on_close_click
            />
            {children()}
        </article>
    }
}

/// An optional message header that can hold a title and a delete element.
/// https://bulma.io/documentation/components/message/
#[component]
pub fn MessageHeader(
    /// Extra classes to apply to the header.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Header children (e.g., title text, a delete button).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || {
            let extra = classes.get();
            if extra.trim().is_empty() {
                "message-header".to_string()
            } else {
                format!("message-header {}", extra)
            }
        }
    };

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

/// A container for the body of a message.
/// https://bulma.io/documentation/components/message/
#[component]
pub fn MessageBody(
    /// Extra classes to apply to the body.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Body children.
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || {
            let extra = classes.get();
            if extra.trim().is_empty() {
                "message-body".to_string()
            } else {
                format!("message-body {}", extra)
            }
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
    fn message_renders_base_class_and_children() {
        let html = view! {
            <Message>
                <p>"Hello"</p>
            </Message>
        }.to_html();

        assert!(html.contains(r#"class="message""#), "expected base 'message' class; got: {}", html);
        assert!(html.contains("Hello"), "expected children rendered; got: {}", html);
    }

    #[test]
    fn message_header_and_body_classes() {
        let html = view! {
            <Message classes="is-primary">
                <MessageHeader><p>"Header"</p></MessageHeader>
                <MessageBody><p>"Body"</p></MessageBody>
            </Message>
        }.to_html();

        assert!(html.contains("message-header"), "expected header class; got: {}", html);
        assert!(html.contains("message-body"), "expected body class; got: {}", html);
        assert!(html.contains(r#"class="message is-primary""#) || html.contains("message is-primary "), "expected color class on message; got: {}", html);
    }
}
