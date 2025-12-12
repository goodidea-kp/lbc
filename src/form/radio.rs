use std::sync::Arc;

use leptos::html;
use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, Get, GetUntracked, IntoView, NodeRef,
    NodeRefAttribute, Signal, component, view,
};

use crate::util::TestAttr;

/// The mutually exclusive radio buttons in their native format.
///
/// https://bulma.io/documentation/form/radio/
///
/// All LBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// NOTE (tachys 0.2.11):
/// - Avoid `on:*` event bindings to prevent "callback removed before attaching" panics.
///   We attach the input listener manually on wasm32.
/// - Avoid reactive attribute closures where possible; compute stable values once.
#[component]
pub fn Radio(
    /// The `name` attribute for this form element.
    ///
    /// All members of the same radio group must have the same value for their `name` attribute.
    #[prop(into)]
    name: Signal<String>,

    /// The `value` attribute for this form element.
    ///
    /// This is different from other form elements, as this value does not change. It represents
    /// the value to be used for the radio group overall when this element is selected.
    #[prop(into)]
    value: Signal<String>,

    /// The value of the currently selected radio of this radio group.
    #[prop(optional)]
    checked_value: Option<String>,

    /// The callback to be used for propagating changes to the selected radio of the radio group.
    update: Arc<dyn Fn(String) + Send + Sync>,

    /// Component children rendered next to the radio input inside the label.
    children: Children,

    /// Extra classes to apply to the outer label.
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
) -> impl IntoView {
    // Compute stable values once to reduce reactive bindings.
    let name_value = name.get_untracked();
    let value_value = value.get_untracked();

    let class_value = {
        let extra = classes.get_untracked().trim().to_string();
        if extra.is_empty() {
            "radio".to_string()
        } else {
            format!("radio {}", extra)
        }
    };

    let is_disabled = disabled.get_untracked();

    let is_checked = checked_value
        .as_ref()
        .is_some_and(|checked_value| checked_value == &value_value);

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:input` and attach the input listener manually on wasm32.
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::Event as WebEvent;

        let has_attached = Rc::new(Cell::new(false));
        let input_ref_for_effect = input_ref.clone();
        let update_for_effect = update.clone();
        let value_for_effect = value_value.clone();

        Effect::new(move |_| {
            if has_attached.get() {
                return;
            }

            let Some(input_element) = input_ref_for_effect.get() else {
                return;
            };

            // Clone inside the effect so the effect closure remains FnMut.
            let update_for_input = update_for_effect.clone();
            let value_for_input = value_for_effect.clone();

            let input_closure: Closure<dyn FnMut(WebEvent)> =
                Closure::wrap(Box::new(move |_event: WebEvent| {
                    (update_for_input)(value_for_input.clone());
                }));

            input_element
                .add_event_listener_with_callback("input", input_closure.as_ref().unchecked_ref())
                .ok();

            has_attached.set(true);
            input_closure.forget();
        });
    }

    view! {
        <label
            class=class_value
            attr:data-testid=data_testid
            attr:data-cy=data_cy
        >
            <input
                node_ref=input_ref
                type="radio"
                name=name_value
                value=value_value
                checked=is_checked
                disabled=is_disabled
            />
            {children()}
        </label>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;
    use std::sync::Arc;

    fn noop() -> Arc<dyn Fn(String) + Send + Sync> {
        Arc::new(|_v| {})
    }

    #[test]
    fn radio_renders_base_class() {
        let html =
            view! { <Radio name="group" value="A" update=noop()>"Option A"</Radio> }.to_html();
        assert!(
            html.contains(r#"class="radio""#),
            "expected base 'radio' class; got: {}",
            html
        );
        assert!(
            html.contains("Option A"),
            "expected children rendered; got: {}",
            html
        );
    }

    #[test]
    fn radio_checked_matches_checked_value() {
        let html = view! { <Radio name="g" value="A" checked_value="A".to_string() update=noop()>"A"</Radio> }.to_html();
        assert!(
            html.contains(r#"checked"#),
            "expected 'checked' present when values match; got: {}",
            html
        );

        let html_unchecked = view! { <Radio name="g" value="A" checked_value="B".to_string() update=noop()>"A"</Radio> }.to_html();
        assert!(
            !html_unchecked.contains(r#"checked"#),
            "did not expect 'checked' when values differ; got: {}",
            html_unchecked
        );
    }

    #[test]
    fn radio_respects_disabled() {
        let html =
            view! { <Radio name="g" value="A" disabled=true update=noop()>"A"</Radio> }.to_html();
        assert!(
            html.contains(r#"disabled"#),
            "expected 'disabled' attribute; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use std::sync::Arc;
    use wasm_bindgen_test::*;

    fn noop() -> Arc<dyn Fn(String) + Send + Sync> {
        Arc::new(|_v| {})
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn radio_renders_test_attr_as_data_testid() {
        let html = view! {
            <Radio name="group" value="A" update=noop() test_attr=TestAttr::test_id("radio-test")>"Option A"</Radio>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="radio-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn radio_no_test_attr_when_not_provided() {
        let html = view! {
            <Radio name="group" value="A" update=noop()>"Option A"</Radio>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute; got: {}",
            html
        );
    }
}
