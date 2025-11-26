use leptos::html;
use leptos::prelude::{
    ClassAttribute, CustomAttribute, Get, IntoAny, IntoView, NodeRef, NodeRefAttribute, OnAttribute,
    Signal, component, event_target_value, view,
};
use std::fmt;
use std::rc::Rc;

use crate::util::{Size, TestAttr};

/// The 5 allowed types for an input component (Bulma-focused).
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Tel,
    Number,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = match self {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Tel => "tel",
            InputType::Number => "number",
        };
        write!(f, "{}", as_str)
    }
}

fn size_class(sz: Size) -> &'static str {
    match sz {
        Size::Small => "is-small",
        Size::Normal => "is-normal",
        Size::Medium => "is-medium",
        Size::Large => "is-large",
    }
}

/// A text input element following Bulma styles.
/// All LBC form components are controlled: the value is provided by a parent,
/// and changes are propagated through the `update` callback.
#[component]
pub fn Input(
    /// The `name` attribute for this form element.
    #[prop(into)]
    name: Signal<String>,

    /// The controlled value of this form element.
    #[prop(into)]
    value: Signal<String>,

    /// The callback used to propagate changes to the parent.
    update: Rc<dyn Fn(String)>,

    /// Extra classes to apply to the input.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// The input type. Defaults to Text when not provided.
    #[prop(optional)]
    r#type: Option<InputType>,

    /// The placeholder value for this component.
    #[prop(optional, into)]
    placeholder: Signal<String>,

    /// The size of this component.
    #[prop(optional)]
    size: Option<Size>,

    /// Use rounded appearance.
    #[prop(optional, into)]
    rounded: Signal<bool>,

    /// Display a loading spinner within this component.
    #[prop(optional, into)]
    loading: Signal<bool>,

    /// Disable this component.
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Make this component read-only.
    #[prop(optional, into)]
    readonly: Signal<bool>,

    /// Make this component static.
    #[prop(optional, into)]
    r#static: Signal<bool>,

    /// Step value for number input. If not provided, defaults to 1.0.
    #[prop(optional)]
    step: Option<f32>,

    /// Optional test attribute (renders as data-* attribute) on the <input>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let input_type = r#type.unwrap_or(InputType::Text);
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    let class = {
        let classes = classes.clone();
        let rounded = rounded.clone();
        let loading = loading.clone();
        let r#static = r#static.clone();
        move || {
            let mut parts = vec!["input".to_string()];

            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if let Some(sz) = size {
                parts.push(size_class(sz).to_string());
            }
            if rounded.get() {
                parts.push("is-rounded".to_string());
            }
            if loading.get() {
                parts.push("is-loading".to_string());
            }
            if r#static.get() {
                parts.push("is-static".to_string());
            }
            parts.join(" ")
        }
    };

    // Text-like handler: forward the current target value to the parent via `update`
    let on_input_text = {
        let update = update.clone();
        move |ev| {
            let new_value = event_target_value(&ev);
            (update)(new_value);
        }
    };

    // Number handler: ensure validity message mirrors Yew behavior, then forward value
    let on_input_number = {
        let update = update.clone();
        let input_ref = input_ref.clone();
        move |ev| {
            let new_value = event_target_value(&ev);
            if let Some(input) = input_ref.get() {
                input.set_custom_validity("");
                let is_valid = input.check_validity();
                if !new_value.trim().is_empty() && !is_valid {
                    input.set_custom_validity(
                        "Please enter a number with up to two decimal places.",
                    );
                }
            }
            (update)(new_value);
        }
    };

    let on_invalid = {
        let input_ref = input_ref.clone();
        move |_| {
            if let Some(input) = input_ref.get() {
                if input.value().is_empty() {
                    input.set_custom_validity("");
                } else {
                    input.set_custom_validity(
                        "Please enter a number with up to two decimal places.",
                    );
                }
            }
        }
    };

    let numeric_step = step.unwrap_or(1.0).to_string();

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        {
            if matches!(input_type, InputType::Number) {
                view! {
                    <input
                        name=name.get()
                        value=value.get()
                        class=move || class()
                        type=input_type.to_string()
                        node_ref=input_ref
                        on:input=on_input_number
                        on:invalid=on_invalid
                        placeholder=placeholder.get()
                        disabled=disabled.get()
                        readonly=readonly.get()
                        step=numeric_step.clone()
                        pattern="[0-9]+([.][0-9]{0,2})?"
                        attr:data-testid=move || data_testid.clone()
                        attr:data-cy=move || data_cy.clone()
                    />
                }.into_any()
            } else {
                view! {
                    <input
                        name=name.get()
                        value=value.get()
                        on:input=on_input_text
                        class=move || class()
                        type=input_type.to_string()
                        placeholder=placeholder.get()
                        disabled=disabled.get()
                        readonly=readonly.get()
                        attr:data-testid=move || data_testid.clone()
                        attr:data-cy=move || data_cy.clone()
                    />
                }.into_any()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    use std::rc::Rc;

    fn noop() -> Rc<dyn Fn(String)> {
        Rc::new(|_value: String| {})
    }

    #[test]
    fn input_renders_default_text_type_and_classes() {
        let html = view! { <Input name="username" value="" update=noop() /> }.to_html();
        assert!(
            html.contains(r#"class="input""#),
            "expected base 'input' class; got: {}",
            html
        );
        assert!(
            html.contains(r#"type="text""#),
            "expected default type=text; got: {}",
            html
        );
        assert!(
            html.contains(r#"name="username""#),
            "expected name attribute; got: {}",
            html
        );
    }

    #[test]
    fn input_with_size_rounded_loading_static_classes() {
        let html = view! {
            <Input
                name="n"
                value="v"
                size=Size::Small
                rounded=true
                loading=true
                r#static=true
                update=noop()
            />
        }
        .to_html();
        assert!(
            html.contains("is-small"),
            "expected size class; got: {}",
            html
        );
        assert!(
            html.contains("is-rounded"),
            "expected rounded class; got: {}",
            html
        );
        assert!(
            html.contains("is-loading"),
            "expected loading class; got: {}",
            html
        );
        assert!(
            html.contains("is-static"),
            "expected static class; got: {}",
            html
        );
    }

    #[test]
    fn number_input_has_pattern_and_step() {
        let html = view! {
            <Input
                name="amount"
                value="0"
                r#type=InputType::Number
                step=1.0
                update=noop()
            />
        }
        .to_html();
        assert!(
            html.contains(r#"type="number""#),
            "expected type=number; got: {}",
            html
        );
        assert!(
            html.contains(r#"pattern="[0-9]+([.][0-9]{0,2})?""#),
            "expected pattern attribute; got: {}",
            html
        );
        assert!(
            html.contains(r#"step="1""#),
            "expected step=1; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::{Size, TestAttr};
    use leptos::prelude::*;
    use std::rc::Rc;
    use wasm_bindgen_test::*;

    fn noop() -> Rc<dyn Fn(String)> {
        Rc::new(|_value: String| {})
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn input_renders_test_attr_as_data_testid() {
        let html = view! {
            <Input name="username" value="" update=noop() test_attr=TestAttr::test_id("input-test") />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="input-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn input_no_test_attr_when_not_provided() {
        let html = view! {
            <Input name="username" value="" update=noop() />
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute; got: {}",
            html
        );
    }
}
