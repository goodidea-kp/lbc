/*!
Leptos version of Bulma Delete element.

Bulma docs: https://bulma.io/documentation/elements/delete/
*/

use leptos::ev::MouseEvent;
use leptos::html;
use leptos::prelude::{
    AnyView, Children, ClassAttribute, CustomAttribute, Effect, ElementChild, Get, IntoAny,
    IntoView, NodeRef, NodeRefAttribute, Signal, component, view,
};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use crate::util::TestAttr;

#[cfg(target_arch = "wasm32")]
fn console_log(message: &str) {
    use leptos::wasm_bindgen::JsValue;
    use leptos::web_sys::console;

    console::log_1(&JsValue::from_str(message));
}

#[cfg(not(target_arch = "wasm32"))]
fn console_log(message: &str) {
    println!("{message}");
}

#[cfg(target_arch = "wasm32")]
fn next_instance_id() -> u32 {
    thread_local! {
        static NEXT_ID: Cell<u32> = Cell::new(1);
    }

    NEXT_ID.with(|cell| {
        let id = cell.get();
        cell.set(id.saturating_add(1));
        id
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn next_instance_id() -> u32 {
    0
}

/// A versatile delete cross.
#[component]
pub fn Delete(
    /// Additional CSS classes to append to the base "delete" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (button, a, span, div)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,
    /// Child content to render inside (usually empty for Bulma delete, but allowed)
    #[prop(optional)]
    children: Option<Children>,
    /// Optional click handler passed through to the rendered element.
    #[prop(optional)]
    on_click: Option<Arc<dyn Fn(MouseEvent) + Send + Sync>>,
    /// Optional test attribute (renders as data-* attribute) on the rendered element.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (for example, `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> AnyView {
    let instance_id = next_instance_id();

    // Build class attribute: "delete [extra classes]"
    let mut class_attr = String::from("delete");

    if let Some(extra) = classes {
        let extra_val = extra.get();
        if !extra_val.trim().is_empty() {
            class_attr.push(' ');
            class_attr.push_str(extra_val.trim());
        }
    }

    let tag_name = tag
        .as_ref()
        .map(|tag| tag.get().to_lowercase())
        .unwrap_or_else(|| "button".to_string());

    console_log(&format!(
        "[Delete#{instance_id}] render start tag='{tag_name}' class='{class_attr}' has_on_click={} has_test_attr={}",
        on_click.is_some(),
        test_attr.is_some()
    ));

    // Render children only if provided; otherwise render nothing.
    let content = match children {
        Some(children) => children(),
        None => view! {}.into_any(),
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach the click listener manually on wasm32.
    //
    // IMPORTANT:
    // `Effect::new` can run multiple times; re-attaching listeners during rebuilds can
    // still trip tachys lifecycle edge cases. We guard to attach only once.
    #[cfg(target_arch = "wasm32")]
    fn attach_click_listener_once<E: Clone + 'static>(
        instance_id: u32,
        element_ref: NodeRef<E>,
        on_click: Option<Arc<dyn Fn(MouseEvent) + Send + Sync>>,
    ) where
        E: Into<leptos::web_sys::EventTarget>,
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys::Event;

        let has_attached = Rc::new(Cell::new(false));
        let on_click_for_effect = on_click.clone();

        Effect::new(move |_| {
            console_log(&format!(
                "[Delete#{instance_id}] effect tick has_attached={} has_on_click={}",
                has_attached.get(),
                on_click_for_effect.is_some()
            ));

            if has_attached.get() {
                console_log(&format!(
                    "[Delete#{instance_id}] effect: already attached, skipping"
                ));
                return;
            }

            let Some(element) = element_ref.get() else {
                console_log(&format!(
                    "[Delete#{instance_id}] effect: element_ref.get() -> None (not mounted yet)"
                ));
                return;
            };

            console_log(&format!(
                "[Delete#{instance_id}] effect: element_ref.get() -> Some(element)"
            ));

            let Some(on_click_callback) = on_click_for_effect.clone() else {
                console_log(&format!(
                    "[Delete#{instance_id}] effect: no on_click callback provided; marking attached to avoid re-run"
                ));
                has_attached.set(true);
                return;
            };

            let event_target: leptos::web_sys::EventTarget = element.into();

            let click_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    console_log(&format!("[Delete#{instance_id}] DOM click handler fired"));

                    let Ok(mouse_event) = event.dyn_into::<MouseEvent>() else {
                        console_log(&format!(
                            "[Delete#{instance_id}] DOM click handler: failed to dyn_into::<MouseEvent>()"
                        ));
                        return;
                    };

                    (on_click_callback)(mouse_event);
                }));

            console_log(&format!(
                "[Delete#{instance_id}] effect: attaching DOM click listener"
            ));

            let attach_result = event_target.add_event_listener_with_callback(
                "click",
                click_closure.as_ref().unchecked_ref(),
            );

            match attach_result {
                Ok(()) => console_log(&format!(
                    "[Delete#{instance_id}] effect: add_event_listener_with_callback OK"
                )),
                Err(error) => console_log(&format!(
                    "[Delete#{instance_id}] effect: add_event_listener_with_callback ERR: {:?}",
                    error
                )),
            }

            // Mark attached and keep closure alive for the lifetime of the page/app.
            has_attached.set(true);
            console_log(&format!(
                "[Delete#{instance_id}] effect: marked attached; leaking closure via forget()"
            ));
            click_closure.forget();
        });
    }

    match tag_name.as_str() {
        "a" => {
            let element_ref: NodeRef<html::A> = NodeRef::new();

            #[cfg(target_arch = "wasm32")]
            attach_click_listener_once(instance_id, element_ref.clone(), on_click.clone());

            view! {
                <a
                    node_ref=element_ref
                    class=class_attr.clone()
                    attr:data-testid=move || data_testid.clone()
                    attr:data-cy=move || data_cy.clone()
                >
                    {content}
                </a>
            }
            .into_any()
        }
        "span" => {
            let element_ref: NodeRef<html::Span> = NodeRef::new();

            #[cfg(target_arch = "wasm32")]
            attach_click_listener_once(instance_id, element_ref.clone(), on_click.clone());

            view! {
                <span
                    node_ref=element_ref
                    class=class_attr.clone()
                    attr:data-testid=move || data_testid.clone()
                    attr:data-cy=move || data_cy.clone()
                >
                    {content}
                </span>
            }
            .into_any()
        }
        "div" => {
            let element_ref: NodeRef<html::Div> = NodeRef::new();

            #[cfg(target_arch = "wasm32")]
            attach_click_listener_once(instance_id, element_ref.clone(), on_click.clone());

            view! {
                <div
                    node_ref=element_ref
                    class=class_attr.clone()
                    attr:data-testid=move || data_testid.clone()
                    attr:data-cy=move || data_cy.clone()
                >
                    {content}
                </div>
            }
            .into_any()
        }
        // default "button"
        _ => {
            let element_ref: NodeRef<html::Button> = NodeRef::new();

            #[cfg(target_arch = "wasm32")]
            attach_click_listener_once(instance_id, element_ref.clone(), on_click.clone());

            view! {
                <button
                    node_ref=element_ref
                    class=class_attr.clone()
                    attr:data-testid=move || data_testid.clone()
                    attr:data-cy=move || data_cy.clone()
                >
                    {content}
                </button>
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
    fn delete_renders_default_button() {
        let html = view! { <Delete></Delete> }.to_html();
        assert!(
            html.contains(r#"class="delete""#),
            "expected base 'delete' class, got: {}",
            html
        );
        assert!(
            html.contains("<button"),
            "expected default button tag, got: {}",
            html
        );
    }

    #[test]
    fn delete_custom_tag_and_classes() {
        let html = view! { <Delete tag="a" classes="is-large"></Delete> }.to_html();
        assert!(
            html.contains(r#"class="delete is-large""#),
            "expected combined classes, got: {}",
            html
        );
        assert!(html.contains("<a"), "expected anchor tag, got: {}", html);
    }

    #[test]
    fn delete_renders_children() {
        let html = view! { <Delete>"X"</Delete> }.to_html();
        assert!(
            html.contains(">X<"),
            "expected child content, got: {}",
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
    fn delete_renders_test_id() {
        let html = view! {
            <Delete test_attr=TestAttr::test_id("delete-test")></Delete>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="delete-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn delete_no_test_attr_when_not_provided() {
        let html = view! { <Delete></Delete> }.to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn delete_accepts_custom_test_attr_key() {
        let html = view! {
            <Delete test_attr=TestAttr::new("data-cy", "delete-cy")></Delete>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="delete-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
