use std::cell::Cell;
use std::rc::Rc;

use leptos::html;
use leptos::prelude::{
    AriaAttributes, Children, ClassAttribute, CustomAttribute, Effect, ElementChild, Get,
    GetUntracked, IntoView, NodeRef, NodeRefAttribute, Set, Signal, StyleAttribute, component,
    view,
};

use crate::util::TestAttr;

fn base_class(extra: &str) -> String {
    if extra.trim().is_empty() {
        "message".to_string()
    } else {
        format!("message {}", extra)
    }
}

/// Colored message blocks, to emphasize part of your page.
/// https://bulma.io/documentation/components/message/
///
/// NOTE (tachys 0.2.11):
/// - Avoid `on:*` event bindings to prevent "callback removed before attaching" panics.
///   We attach DOM listeners manually on wasm32.
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

    /// Optional test attribute (renders as data-* attribute) on the root <article>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    /// Child content of the message (usually MessageHeader and MessageBody).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class(&classes.get())
    };

    let (is_closed, set_is_closed) = leptos::prelude::signal(false);

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listener manually on wasm32.
    let close_button_ref: NodeRef<html::Button> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys::Event;

        let has_attached = Rc::new(Cell::new(false));
        let close_button_ref_for_effect = close_button_ref.clone();
        let on_close_for_effect = on_close.clone();
        let closable_for_effect = closable.clone();
        let is_closed_for_effect = is_closed.clone();
        let set_is_closed_for_effect = set_is_closed.clone();

        Effect::new(move |_| {
            if has_attached.get() {
                return;
            }

            let Some(button_element) = close_button_ref_for_effect.get() else {
                return;
            };

            // Clone the callback into the event handler so the Effect closure remains FnMut.
            let on_close_for_click = on_close_for_effect.clone();
            let closable_for_click = closable_for_effect.clone();
            let is_closed_for_click = is_closed_for_effect.clone();
            let set_is_closed_for_click = set_is_closed_for_effect.clone();

            let click_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    event.prevent_default();

                    // If the close button isn't currently visible, ignore clicks.
                    if !closable_for_click.get_untracked() || is_closed_for_click.get_untracked() {
                        return;
                    }

                    if let Some(callback) = on_close_for_click.as_ref() {
                        callback();
                    } else {
                        set_is_closed_for_click.set(true);
                    }
                }));

            button_element
                .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
                .ok();

            has_attached.set(true);
            click_closure.forget();
        });
    }

    view! {
        <article
            class=class
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
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
                node_ref=close_button_ref
                class="delete is-small"
                aria-label="delete"
                type="button"
                style=move || if closable.get() && !is_closed.get() {
                    "position:absolute; right:0.5rem; top:0.5rem; z-index: 10;"
                } else {
                    "display: none;"
                }
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

    /// Optional test attribute (renders as data-* attribute) on the header <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

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

/// A container for the body of a message.
/// https://bulma.io/documentation/components/message/
#[component]
pub fn MessageBody(
    /// Extra classes to apply to the body.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test attribute (renders as data-* attribute) on the body <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

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
    fn message_renders_base_class_and_children() {
        let html = view! {
            <Message>
                <p>"Hello"</p>
            </Message>
        }
        .to_html();

        assert!(
            html.contains(r#"class="message""#),
            "expected base 'message' class; got: {}",
            html
        );
        assert!(
            html.contains("Hello"),
            "expected children rendered; got: {}",
            html
        );
    }

    #[test]
    fn message_header_and_body_classes() {
        let html = view! {
            <Message classes="is-primary">
                <MessageHeader><p>"Header"</p></MessageHeader>
                <MessageBody><p>"Body"</p></MessageBody>
            </Message>
        }
        .to_html();

        assert!(
            html.contains("message-header"),
            "expected header class; got: {}",
            html
        );
        assert!(
            html.contains("message-body"),
            "expected body class; got: {}",
            html
        );
        assert!(
            html.contains(r#"class="message is-primary""#) || html.contains("message is-primary "),
            "expected color class on message; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use leptos::prelude::*;
    use std::rc::Rc;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn noop_close() -> Option<Rc<dyn Fn()>> {
        Some(Rc::new(|| {}))
    }

    #[wasm_bindgen_test]
    fn message_renders_test_attr_as_data_testid() {
        let html = view! {
            <Message classes="is-primary" test_attr="message-test">
                <MessageBody><p>"Body"</p></MessageBody>
            </Message>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="message-test""#),
            "expected data-testid attribute on Message; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn message_no_test_attr_when_not_provided() {
        let html = view! {
            <Message>
                <MessageBody><p>"Body"</p></MessageBody>
            </Message>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute on Message when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn message_header_renders_test_attr_as_data_testid() {
        let html = view! {
            <MessageHeader classes="extra" test_attr="message-header-test">
                <p>"Header"</p>
            </MessageHeader>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="message-header-test""#),
            "expected data-testid attribute on MessageHeader; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn message_body_renders_test_attr_as_data_testid() {
        let html = view! {
            <MessageBody classes="extra" test_attr="message-body-test">
                <p>"Body"</p>
            </MessageBody>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="message-body-test""#),
            "expected data-testid attribute on MessageBody; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn message_closable_renders_delete_button() {
        let html = view! {
            <Message classes="is-primary" closable=true on_close=noop_close()>
                <MessageBody><p>"Body"</p></MessageBody>
            </Message>
        }
        .to_html();

        assert!(
            html.contains(r#"class="delete is-small""#),
            "expected delete button when closable=true; got: {}",
            html
        );
    }
}
