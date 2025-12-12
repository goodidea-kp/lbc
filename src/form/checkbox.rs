use std::sync::Arc;

use leptos::html;
use leptos::prelude::*;

use crate::util::TestAttr;

/// The 2-state checkbox in its native Bulma format.
///
/// https://bulma.io/documentation/form/checkbox/
///
/// Controlled component:
/// - `checked` is the current value (supports static bool or reactive signal).
/// - `update` is an optional callback invoked with the next value when the user clicks.
///
/// NOTE (tachys 0.2.11):
/// - Avoid reactive property bindings for `checked` to prevent "property removed early" panics.
/// - Avoid `on:*` event bindings to prevent "callback removed before attaching" panics.
///   We attach the click listener manually on wasm32.
#[component]
pub fn Checkbox(
    /// The `name` attribute for this form element.
    #[prop(into)]
    name: String,

    /// The controlled value of this form element.
    ///
    /// Accepts a bool or a reactive signal.
    #[prop(into)]
    checked: Signal<bool>,

    /// Optional callback to propagate changes to the parent with the new value.
    #[prop(optional)]
    update: Option<Arc<dyn Fn(bool) + Send + Sync>>,

    /// Additional CSS classes to append to Bulma's "checkbox".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Disable this component.
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Optional test attribute (renders as data-* attribute) on the outer <label>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    /// Label/content shown next to the checkbox.
    children: Children,
) -> impl IntoView {
    let class = move || {
        let extra = classes.get();
        if extra.trim().is_empty() {
            "checkbox".to_string()
        } else {
            format!("checkbox {}", extra)
        }
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    // IMPORTANT:
    // Do not bind `checked` reactively (even via `checked=move || checked.get()`), because
    // tachys may treat it as a property binding and panic "property removed early".
    // We set the initial checked state non-reactively and rely on the parent to re-render
    // the component when `checked` changes.
    let initial_checked = checked.get_untracked();

    // Workaround for tachys 0.2.11:
    // - avoid `on:click`
    // - attach click listener manually on wasm32
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::{Event, HtmlInputElement};

        let has_attached = Rc::new(Cell::new(false));
        let input_ref_for_effect = input_ref.clone();
        let checked_for_effect = checked.clone();
        let update_for_effect = update.clone();

        Effect::new(move |_| {
            if has_attached.get() {
                return;
            }

            let Some(input_element) = input_ref_for_effect.get() else {
                return;
            };

            let Some(update_callback) = update_for_effect.clone() else {
                has_attached.set(true);
                return;
            };

            let click_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    let target_input = event
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                    let Some(_target_input) = target_input else {
                        return;
                    };

                    // Controlled component: compute next value from current signal.
                    // Use `get_untracked()` to avoid reactive-graph warnings about reading
                    // signals outside a tracking context.
                    let next_value = !checked_for_effect.get_untracked();
                    (update_callback)(next_value);
                }));

            input_element
                .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
                .ok();

            has_attached.set(true);
            click_closure.forget();
        });
    }

    view! {
        <label
            class=class
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            <input
                node_ref=input_ref
                type="checkbox"
                name=name.clone()
                checked=initial_checked
                disabled=disabled.get_untracked()
            />
            {children()}
        </label>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn checkbox_renders_base_class_and_label_text() {
        let html = view! { <Checkbox name="agree" checked=true>{"Agree"}</Checkbox> }.to_html();
        assert!(
            html.contains(r#"class="checkbox""#),
            "expected 'checkbox' class in: {}",
            html
        );
        assert!(html.contains(">Agree<"), "expected label text in: {}", html);
    }

    #[test]
    fn checkbox_sets_name_attribute() {
        let html = view! { <Checkbox name="terms" checked=false>{"X"}</Checkbox> }.to_html();
        assert!(
            html.contains(r#"name="terms""#),
            "expected name attribute in: {}",
            html
        );
    }

    #[test]
    fn checkbox_allows_extra_classes() {
        let html =
            view! { <Checkbox name="c" checked=true classes="is-small custom">"X"</Checkbox> }
                .to_html();
        assert!(
            html.contains(r#"class="checkbox is-small custom""#),
            "expected merged classes in: {}",
            html
        );
    }

    #[test]
    fn checkbox_can_be_disabled() {
        let html =
            view! { <Checkbox name="d" checked=false disabled=true>"X"</Checkbox> }.to_html();
        assert!(
            html.contains(r#"disabled"#),
            "expected disabled attribute on input in: {}",
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
    fn checkbox_renders_test_attr_as_data_testid() {
        let html = view! {
            <Checkbox name="agree" checked=true test_attr=TestAttr::test_id("checkbox-test")>"Agree"</Checkbox>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="checkbox-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn checkbox_no_test_attr_when_not_provided() {
        let html = view! {
            <Checkbox name="agree" checked=true>"Agree"</Checkbox>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute; got: {}",
            html
        );
    }
}
