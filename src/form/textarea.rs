use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use leptos::html;
use leptos::prelude::{
    ClassAttribute, CustomAttribute, Effect, ElementChild, Get, GetUntracked, IntoAny, IntoView,
    NodeRef, NodeRefAttribute, Signal, StyleAttribute, component, view,
};

use crate::elements::icon::Icon;
use crate::util::{Size, TestAttr};

fn size_class(size: Size) -> &'static str {
    match size {
        Size::Small => "is-small",
        Size::Normal => "is-normal",
        Size::Medium => "is-medium",
        Size::Large => "is-large",
    }
}

/// A multiline textarea component following Bulma styles.
///
/// https://bulma.io/documentation/form/textarea/
///
/// Controlled component: the value comes from a parent, changes are propagated via `update`.
///
/// NOTE (tachys 0.2.11):
/// - Avoid `prop:value` to prevent "property removed early" panics.
/// - Avoid `on:*` event bindings to prevent "callback removed before attaching" panics.
///   We attach DOM listeners manually on wasm32.
/// - We intentionally leak JS closures via `forget()` to avoid `on_cleanup` Send+Sync bounds.
#[component]
pub fn TextArea(
    /// The `name` attribute for this form element.
    #[prop(into)]
    name: Signal<String>,

    /// The controlled value of this form element.
    #[prop(into)]
    value: Signal<String>,

    /// The callback to be used for propagating changes to this element's value.
    update: Arc<dyn Fn(String) + Send + Sync>,

    /// Extra classes to apply to the textarea.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// The placeholder value for this component.
    #[prop(optional, into)]
    placeholder: Signal<String>,

    /// The number of rows to which this component will be locked.
    #[prop(optional)]
    rows: Option<u32>,

    /// The size of this component.
    #[prop(optional)]
    size: Option<Size>,

    /// Fix the size of this component.
    #[prop(optional, into)]
    fixed_size: Signal<bool>,

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

    /// Show GenAI ribbon icon (cosmetic helper).
    #[prop(optional, into)]
    is_genai: Signal<bool>,

    /// Optional test attribute (renders as data-* attribute) on the root element:
    /// - when `is_genai=true`, on the wrapping <div>
    /// - otherwise, on the <textarea> itself.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let loading = loading.clone();
        let fixed_size = fixed_size.clone();
        let r#static = r#static.clone();
        move || {
            let mut parts = vec!["textarea".to_string()];

            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if let Some(sz) = size {
                parts.push(size_class(sz).to_string());
            }
            if loading.get() {
                parts.push("is-loading".to_string());
            }
            if r#static.get() {
                parts.push("is-static".to_string());
            }
            if fixed_size.get() {
                parts.push("has-fixed-size".to_string());
            }

            parts.join(" ")
        }
    };

    // Derive specific optional attributes that our macro can render.
    let (data_testid_opt, data_cy_opt) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    // Compute stable values once to avoid reactive property bindings that can panic in tachys.
    let name_value = name.get_untracked();
    let placeholder_value = placeholder.get_untracked();
    let is_disabled = disabled.get_untracked();
    let is_readonly = readonly.get_untracked();
    let rows_value = rows.unwrap_or(0).to_string();

    // Snapshot initial value once; we will apply it on mount via DOM API.
    // This avoids using `value=` in the view macro (not supported for <textarea> in Leptos 0.8).
    let initial_value = value.get_untracked();

    // Workaround for tachys 0.2.11:
    // avoid `on:input` and attach the input listener manually on wasm32.
    let textarea_ref: NodeRef<html::Textarea> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys::{Event, HtmlTextAreaElement};

        let has_attached = Rc::new(Cell::new(false));
        let textarea_ref_for_effect = textarea_ref.clone();
        let update_for_effect = update.clone();
        let initial_value_for_effect = initial_value.clone();

        Effect::new(move |_| {
            if has_attached.get() {
                return;
            }

            let Some(textarea_element) = textarea_ref_for_effect.get() else {
                return;
            };

            let textarea_element: HtmlTextAreaElement = textarea_element.into();

            // Apply initial value after mount.
            textarea_element.set_value(&initial_value_for_effect);

            let update_for_input = update_for_effect.clone();
            let input_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    let target_textarea = event
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlTextAreaElement>().ok());

                    let Some(target_textarea) = target_textarea else {
                        return;
                    };

                    (update_for_input)(target_textarea.value());
                }));

            textarea_element
                .add_event_listener_with_callback("input", input_closure.as_ref().unchecked_ref())
                .ok();

            has_attached.set(true);
            input_closure.forget();
        });
    }

    // Render an optional "GenAI ribbon" icon overlay if requested.
    move || {
        // Clone the attribute values into locals each render so inner closures can move/clone them
        let data_testid = data_testid_opt.clone();
        let data_cy = data_cy_opt.clone();

        if is_genai.get() {
            view! {
                <div
                    id="context"
                    style="position:relative"
                    attr:data-testid=move || data_testid.clone()
                    attr:data-cy=move || data_cy.clone()
                >
                    <Icon size=Size::Small classes="is-pulled-right ribbon">
                        <i class="fa-brands fa-openai"></i>
                    </Icon>
                    <textarea
                        node_ref=textarea_ref
                        name=name_value.clone()
                        class=move || class()
                        placeholder=placeholder_value.clone()
                        disabled=is_disabled
                        readonly=is_readonly
                        rows=rows_value.clone()
                    >
                        {initial_value.clone()}
                    </textarea>
                </div>
            }
            .into_any()
        } else {
            view! {
                <textarea
                    node_ref=textarea_ref
                    name=name_value.clone()
                    class=move || class()
                    placeholder=placeholder_value.clone()
                    disabled=is_disabled
                    readonly=is_readonly
                    rows=rows_value.clone()
                    attr:data-testid=move || data_testid.clone()
                    attr:data-cy=move || data_cy.clone()
                >
                    {initial_value.clone()}
                </textarea>
            }
            .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Size;
    use leptos::prelude::RenderHtml;

    use std::sync::Arc;

    fn noop() -> Arc<dyn Fn(String) + Send + Sync> {
        Arc::new(|_v| {})
    }

    #[test]
    fn textarea_renders_default_class() {
        let html = view! { <TextArea name="notes" value="" update=noop() /> }.to_html();
        assert!(
            html.contains(r#"class="textarea""#),
            "expected base 'textarea' class; got: {}",
            html
        );
        assert!(
            html.contains(r#"name="notes""#),
            "expected name attribute; got: {}",
            html
        );
    }

    #[test]
    fn textarea_loading_size_and_fixed() {
        let html = view! {
            <TextArea
                name="n"
                value="v"
                size=Size::Small
                loading=true
                fixed_size=true
                update=noop()
            />
        }
        .to_html();
        assert!(
            html.contains("is-loading"),
            "expected is-loading; got: {}",
            html
        );
        assert!(
            html.contains("is-small"),
            "expected is-small; got: {}",
            html
        );
        assert!(
            html.contains("has-fixed-size"),
            "expected has-fixed-size; got: {}",
            html
        );
    }

    #[test]
    fn textarea_rows_and_placeholder_and_flags() {
        let html = view! {
            <TextArea
                name="n"
                value="v"
                rows=6
                placeholder="type here"
                disabled=true
                readonly=true
                update=noop()
            />
        }
        .to_html();
        assert!(
            html.contains(r#"rows="6""#),
            "expected rows attr; got: {}",
            html
        );
        assert!(
            html.contains(r#"placeholder="type here""#),
            "expected placeholder; got: {}",
            html
        );
        assert!(
            html.contains("disabled"),
            "expected disabled; got: {}",
            html
        );
        assert!(
            html.contains("readonly"),
            "expected readonly; got: {}",
            html
        );
    }

    #[test]
    fn textarea_genai_ribbon() {
        let html = view! { <TextArea name="g" value="" is_genai=true update=noop() /> }.to_html();
        assert!(
            html.contains("ribbon"),
            "expected ribbon icon when is_genai; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::{Size, TestAttr};
    use leptos::prelude::*;
    use std::sync::Arc;
    use wasm_bindgen_test::*;

    fn noop() -> Arc<dyn Fn(String) + Send + Sync> {
        Arc::new(|_v| {})
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn textarea_renders_test_attr_as_data_testid() {
        let html = view! {
            <TextArea name="notes" value="" update=noop() test_attr=TestAttr::test_id("textarea-test") />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="textarea-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn textarea_no_test_attr_when_not_provided() {
        let html = view! { <TextArea name="notes" value="" update=noop() /> }.to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute; got: {}",
            html
        );
    }
}
