use std::cell::Cell;
use std::rc::Rc;

use crate::util::TestAttr;
use leptos::html;
use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, Effect, ElementChild, Get, GetUntracked,
    GlobalAttributes, IntoAny, IntoView, NodeRef, NodeRefAttribute, Set, Signal, StyleAttribute,
    component, view,
};

/// A Bulma dropdown menu with a trigger button.
/// https://bulma.io/documentation/components/dropdown/
#[component]
pub fn Dropdown(
    /// Extra classes to apply to the root "dropdown" container.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Make this dropdown triggerable based on hover (CSS only).
    #[prop(optional, into)]
    hoverable: Signal<bool>,

    /// Extra classes to apply to the trigger Button.
    #[prop(optional, into)]
    button_classes: Signal<String>,

    /// Content placed inside the trigger Button.
    button: Children,

    /// Content placed inside the dropdown-content container.
    children: Children,

    /// Optional test attribute (renders as data-* attribute) on the root <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let (is_active, set_is_active) = leptos::prelude::signal(false);

    let class = {
        let classes = classes.clone();
        let hoverable = hoverable.clone();
        move || {
            let mut parts = vec!["dropdown".to_string()];
            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if hoverable.get() {
                parts.push("is-hoverable".to_string());
            }
            if is_active.get() {
                parts.push("is-active".to_string());
            }
            parts.join(" ")
        }
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually on wasm32.
    let trigger_button_ref: NodeRef<html::Button> = NodeRef::new();
    let overlay_ref: NodeRef<html::Div> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys::Event;

        let trigger_attached = Rc::new(Cell::new(false));
        let overlay_attached = Rc::new(Cell::new(false));

        let trigger_button_ref_for_effect = trigger_button_ref.clone();
        let overlay_ref_for_effect = overlay_ref.clone();

        let hoverable_for_effect = hoverable.clone();
        let is_active_for_effect = is_active.clone();
        let set_is_active_for_effect = set_is_active.clone();

        Effect::new(move |_| {
            // Attach trigger click once.
            if !trigger_attached.get() {
                if let Some(button_element) = trigger_button_ref_for_effect.get() {
                    let hoverable_for_click = hoverable_for_effect.clone();
                    let set_is_active_for_click = set_is_active_for_effect.clone();

                    let click_closure: Closure<dyn FnMut(Event)> =
                        Closure::wrap(Box::new(move |event: Event| {
                            event.prevent_default();
                            if !hoverable_for_click.get_untracked() {
                                set_is_active_for_click.set(true);
                            }
                        }));

                    button_element
                        .add_event_listener_with_callback(
                            "click",
                            click_closure.as_ref().unchecked_ref(),
                        )
                        .ok();

                    trigger_attached.set(true);
                    click_closure.forget();
                }
            }

            // Attach overlay click once (closes dropdown).
            if !overlay_attached.get() {
                if let Some(overlay_element) = overlay_ref_for_effect.get() {
                    let set_is_active_for_click = set_is_active_for_effect.clone();

                    let click_closure: Closure<dyn FnMut(Event)> =
                        Closure::wrap(Box::new(move |event: Event| {
                            event.prevent_default();
                            set_is_active_for_click.set(false);
                        }));

                    overlay_element
                        .add_event_listener_with_callback(
                            "click",
                            click_closure.as_ref().unchecked_ref(),
                        )
                        .ok();

                    overlay_attached.set(true);
                    click_closure.forget();
                }
            }

            // If the dropdown is not active, the overlay isn't rendered, so `overlay_ref.get()`
            // will be None. That's fine; the effect will run again when it appears.
            let _ = is_active_for_effect.get();
        });
    }

    view! {
        <div
            class=move || class()
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {move || if is_active.get() && !hoverable.get() {
                // overlay to close when clicking outside
                view! {
                    <div
                        node_ref=overlay_ref
                        style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"
                    ></div>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}

            <div class="dropdown-trigger">
                <button
                    node_ref=trigger_button_ref
                    class=move || {
                        let extra = button_classes.get();
                        if extra.trim().is_empty() {
                            "button".to_string()
                        } else {
                            format!("button {}", extra)
                        }
                    }
                    type="button"
                >
                    {button()}
                </button>
            </div>

            <div class="dropdown-menu" role="menu" style="position: relative; z-index: 20;">
                <div class="dropdown-content">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn dropdown_renders_base_structure() {
        let html = view! {
            <Dropdown button=Box::new(|| view!{ "Open" }.into_any())>
                <a class="dropdown-item">"Item 1"</a>
                <a class="dropdown-item">"Item 2"</a>
            </Dropdown>
        }
        .to_html();

        assert!(
            html.contains(r#"class="dropdown""#),
            "expected base 'dropdown' class; got: {}",
            html
        );
        assert!(
            html.contains("dropdown-menu") && html.contains("dropdown-content"),
            "expected dropdown structure; got: {}",
            html
        );
        assert!(
            html.contains("Open"),
            "expected button content rendered; got: {}",
            html
        );
        assert!(
            html.contains("Item 1") && html.contains("Item 2"),
            "expected children rendered; got: {}",
            html
        );
    }

    #[test]
    fn dropdown_hoverable_adds_class() {
        let html = view! {
            <Dropdown hoverable=true button=Box::new(|| view!{ "Btn" }.into_any())>
                <a class="dropdown-item">"X"</a>
            </Dropdown>
        }
        .to_html();

        assert!(
            html.contains("is-hoverable"),
            "expected is-hoverable class; got: {}",
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

    fn trigger() -> Children {
        Box::new(|| view! { "Open" }.into_any())
    }

    #[wasm_bindgen_test]
    fn dropdown_renders_test_attr_as_data_testid() {
        let html = view! {
            <Dropdown
                classes="is-right"
                hoverable=true
                button_classes="is-primary"
                button=trigger()
                test_attr="dropdown-test"
            >
                <a class="dropdown-item">"Item"</a>
            </Dropdown>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="dropdown-test""#),
            "expected data-testid attribute on Dropdown; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn dropdown_no_test_attr_when_not_provided() {
        let html = view! {
            <Dropdown button=trigger()>
                <a class="dropdown-item">"Item"</a>
            </Dropdown>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute on Dropdown when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn dropdown_accepts_custom_test_attr_key() {
        let html = view! {
            <Dropdown
                classes="is-right"
                hoverable=true
                button_classes="is-primary"
                button=trigger()
                test_attr=TestAttr::new("data-cy", "dropdown-cy")
            >
                <a class="dropdown-item">"Item"</a>
            </Dropdown>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="dropdown-cy""#),
            "expected custom data-cy attribute on Dropdown; got: {}",
            html
        );
    }
}
