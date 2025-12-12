use leptos::html;
use leptos::prelude::Effect;
use leptos::prelude::{
    ClassAttribute, CustomAttribute, Get, GetUntracked, IntoAny, IntoView, NodeRef,
    NodeRefAttribute, Signal, component, view,
};

use crate::lbc_log;
use crate::util::{Size, TestAttr};

use std::fmt;
use std::sync::Arc;

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

fn size_class(size: Size) -> &'static str {
    match size {
        Size::Small => "is-small",
        Size::Normal => "is-normal",
        Size::Medium => "is-medium",
        Size::Large => "is-large",
    }
}

/// A text input element following Bulma styles.
/// All LBC form components are controlled: the value is provided by a parent,
/// and changes are propagated through the `update` callback.
///
/// NOTE ABOUT EVENT HANDLING (tachys 0.2.11):
/// We intentionally avoid `on:*` event bindings here because tachys can panic
/// with "callback removed before attaching" during route transitions/rebuilds.
/// Instead, on wasm32 we attach DOM listeners manually after mount.
///
/// NOTE ABOUT CLEANUP:
/// `on_cleanup` requires `Send + Sync`, but `web_sys`/`wasm_bindgen::Closure` are not
/// `Send`/`Sync`. To keep builds working on wasm32, we intentionally leak the JS
/// closures via `forget()` after attaching. This avoids the Send/Sync bound and
/// prevents the tachys panic path.
#[component]
pub fn Input(
    /// The `name` attribute for this form element.
    #[prop(into)]
    name: Signal<String>,

    /// The controlled value of this form element.
    #[prop(into)]
    value: Signal<String>,

    /// The callback used to propagate changes to the parent.
    update: Arc<dyn Fn(String) + Send + Sync + 'static>,

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

    // We always use a NodeRef so we can attach DOM listeners on wasm32.
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    // Avoid capturing reactive signals in event handlers; keep a plain String for logs/attrs.
    let name_for_logs = name.get_untracked();

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
            if let Some(size) = size {
                parts.push(size_class(size).to_string());
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

    let numeric_step = step.unwrap_or(1.0).to_string();

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    lbc_log!(
        "<Input> render name='{}' type='{}' initial='{}'",
        name_for_logs,
        input_type,
        value.get_untracked()
    );

    // Attach DOM listeners manually on wasm32 to avoid tachys event attachment panics.
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::{Event, HtmlInputElement};

        let update_for_effect = Arc::clone(&update);
        let input_ref_for_effect = input_ref.clone();
        let name_for_logs_for_effect = name_for_logs.clone();
        let input_type_for_effect = input_type;

        Effect::new(move |_| {
            let Some(input_element) = input_ref_for_effect.get() else {
                // Not mounted yet; nothing to attach.
                return;
            };

            let input_element: HtmlInputElement = input_element.into();

            // "input" listener
            let update_for_input = Arc::clone(&update_for_effect);
            let name_for_logs_for_input = name_for_logs_for_effect.clone();
            let input_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    let target_input = event
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                    let Some(target_input) = target_input else {
                        return;
                    };

                    let new_value: String = target_input.value();

                    if matches!(input_type_for_effect, InputType::Number) {
                        target_input.set_custom_validity("");
                        let is_valid = target_input.check_validity();
                        if !new_value.trim().is_empty() && !is_valid {
                            target_input.set_custom_validity(
                                "Please enter a number with up to two decimal places.",
                            );
                        }
                        lbc_log!(
                            "<Input> DOM input (number) name='{}' -> '{}' | valid={}",
                            name_for_logs_for_input,
                            new_value,
                            is_valid
                        );
                    } else {
                        lbc_log!(
                            "<Input> DOM input (text) name='{}' -> '{}'",
                            name_for_logs_for_input,
                            new_value
                        );
                    }

                    (update_for_input)(new_value);
                }));

            input_element
                .add_event_listener_with_callback("input", input_closure.as_ref().unchecked_ref())
                .ok();

            // "invalid" listener (only meaningful for number input)
            let invalid_closure: Option<Closure<dyn FnMut(Event)>> =
                if matches!(input_type_for_effect, InputType::Number) {
                    let name_for_logs_for_invalid = name_for_logs_for_effect.clone();
                    Some(Closure::wrap(Box::new(move |event: Event| {
                        let target_input = event
                            .target()
                            .and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                        let Some(target_input) = target_input else {
                            return;
                        };

                        if target_input.value().is_empty() {
                            target_input.set_custom_validity("");
                        } else {
                            target_input.set_custom_validity(
                                "Please enter a number with up to two decimal places.",
                            );
                        }

                        lbc_log!(
                            "<Input> DOM invalid name='{}' current='{}'",
                            name_for_logs_for_invalid,
                            target_input.value()
                        );
                    })))
                } else {
                    None
                };

            if let Some(invalid_closure) = invalid_closure.as_ref() {
                input_element
                    .add_event_listener_with_callback(
                        "invalid",
                        invalid_closure.as_ref().unchecked_ref(),
                    )
                    .ok();
            }

            // Keep closures alive for the lifetime of the page/app.
            // This avoids `on_cleanup`'s Send+Sync requirement (JS values are not Send/Sync).
            input_closure.forget();
            if let Some(invalid_closure) = invalid_closure {
                invalid_closure.forget();
            }
        });
    }

    // NOTE:
    // We intentionally bind `value` as an attribute (not `prop:value`) to avoid
    // tachys "property removed early" panics introduced/triggered by newer versions.
    view! {
        {
            if matches!(input_type, InputType::Number) {
                view! {
                    <input
                        name=name_for_logs.clone()
                        value=move || value.get()
                        class=move || class()
                        type=input_type.to_string()
                        node_ref=input_ref
                        placeholder=placeholder.get_untracked()
                        disabled=disabled.get_untracked()
                        readonly=readonly.get_untracked()
                        step=numeric_step.clone()
                        pattern="[0-9]+([.][0-9]{0,2})?"
                        attr:data-testid=move || data_testid.clone()
                        attr:data-cy=move || data_cy.clone()
                    />
                }
                .into_any()
            } else {
                view! {
                    <input
                        name=name_for_logs.clone()
                        value=move || value.get()
                        class=move || class()
                        type=input_type.to_string()
                        node_ref=input_ref
                        placeholder=placeholder.get_untracked()
                        disabled=disabled.get_untracked()
                        readonly=readonly.get_untracked()
                        attr:data-testid=move || data_testid.clone()
                        attr:data-cy=move || data_cy.clone()
                    />
                }
                .into_any()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    use std::sync::Arc;

    fn noop() -> Arc<dyn Fn(String) + Send + Sync + 'static> {
        Arc::new(|_value: String| {})
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
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use std::sync::Arc;
    use wasm_bindgen_test::*;

    fn noop() -> Arc<dyn Fn(String) + Send + Sync + 'static> {
        Arc::new(|_value: String| {})
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
        let html = view! { <Input name="username" value="" update=noop() /> }.to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute; got: {}",
            html
        );
    }
}
