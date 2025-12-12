use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use leptos::html;
use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, Effect, ElementChild, Get, GetUntracked, IntoView,
    NodeRef, NodeRefAttribute, Signal, component, view,
};

use crate::util::{Size, TestAttr};

fn size_class(size: Size) -> &'static str {
    match size {
        Size::Large => "is-large",
        Size::Medium => "is-medium",
        Size::Normal => "is-normal",
        Size::Small => "is-small",
    }
}

/// A wrapper around an HTML select tag.
///
/// https://bulma.io/documentation/form/select/
///
/// All LBC form components are controlled components. The value comes from a parent,
/// and changes are propagated via the `update` callback.
///
/// NOTE (tachys 0.2.11):
/// - Avoid `prop:value` to prevent "property removed early" panics.
/// - Avoid `on:*` event bindings to prevent "callback removed before attaching" panics.
///   We attach DOM listeners manually on wasm32.
/// - Avoid reactive wrapper attribute closures where possible.
#[component]
pub fn Select(
    /// The `name` attribute for this form element.
    #[prop(into)]
    name: Signal<String>,

    /// The controlled value of this form element.
    #[prop(into)]
    value: Signal<String>,

    /// The callback to be used for propagating changes to this element's value.
    update: Arc<dyn Fn(String) + Send + Sync>,

    /// The `option` and `optgroup` tags of this select component.
    children: Children,

    /// Extra classes to apply to the Bulma "select" wrapper.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// The size of this component.
    #[prop(optional)]
    size: Option<Size>,

    /// Display a loading spinner within this component.
    #[prop(optional, into)]
    loading: Signal<bool>,

    /// Disable this component.
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Optional test attribute (renders as data-* attribute) on the wrapper <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (e.g., `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    // Compute wrapper attributes once (safe mode).
    let mut wrapper_classes = vec!["select".to_string()];

    let extra = classes.get_untracked();
    if !extra.trim().is_empty() {
        wrapper_classes.push(extra);
    }
    if let Some(sz) = size {
        wrapper_classes.push(size_class(sz).to_string());
    }
    if loading.get_untracked() {
        wrapper_classes.push("is-loading".to_string());
    }

    let wrapper_class = wrapper_classes.join(" ");

    let name_value = name.get_untracked();
    let initial_value = value.get_untracked();
    let is_disabled = disabled.get_untracked();

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    let select_ref: NodeRef<html::Select> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys::{Event, HtmlSelectElement};

        let has_attached = Rc::new(Cell::new(false));
        let select_ref_for_effect = select_ref.clone();
        let update_for_effect = update.clone();

        Effect::new(move |_| {
            if has_attached.get() {
                return;
            }

            let Some(select_element) = select_ref_for_effect.get() else {
                return;
            };

            let update_for_input = update_for_effect.clone();

            let input_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    let target_select = event
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlSelectElement>().ok());

                    let Some(target_select) = target_select else {
                        return;
                    };

                    (update_for_input)(target_select.value());
                }));

            select_element
                .add_event_listener_with_callback("input", input_closure.as_ref().unchecked_ref())
                .ok();

            has_attached.set(true);
            input_closure.forget();
        });
    }

    view! {
        <div
            class=wrapper_class
            attr:data-testid=data_testid
            attr:data-cy=data_cy
        >
            <select
                node_ref=select_ref
                name=name_value
                value=initial_value
                disabled=is_disabled
            >
                {children()}
            </select>
        </div>
    }
}

/// A wrapper around an HTML select tag with the `multiple=true` attribute.
///
/// https://bulma.io/documentation/form/select/
///
/// Controlled component: values come from a parent; updates are sent via `update`.
///
/// NOTE (tachys 0.2.11):
/// - Avoid `prop:value` and `on:*` event bindings; attach DOM listeners manually on wasm32.
/// - Compute wrapper attributes once (safe mode).
#[component]
pub fn MultiSelect(
    /// The `name` attribute for this form element.
    #[prop(into)]
    name: Signal<String>,

    /// The controlled values of this form element.
    #[prop(into)]
    value: Signal<Vec<String>>,

    /// The callback to be used for propagating changes to this element's value.
    update: Arc<dyn Fn(Vec<String>) + Send + Sync>,

    /// The `option` and `optgroup` tags of this select component.
    children: Children,

    /// Extra classes to apply to the Bulma "select" wrapper.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// The size of this component.
    #[prop(optional)]
    size: Option<Size>,

    /// Size of the list to display. Defaults to 4.
    #[prop(optional)]
    list_size: Option<u32>,

    /// Display a loading spinner within this component.
    #[prop(optional, into)]
    loading: Signal<bool>,

    /// Disable this component.
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Optional test attribute (renders as data-* attribute) on the wrapper <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (e.g., `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    // Compute wrapper attributes once (safe mode).
    let mut wrapper_classes = vec!["select".to_string(), "is-multiple".to_string()];

    let extra = classes.get_untracked();
    if !extra.trim().is_empty() {
        wrapper_classes.push(extra);
    }
    if let Some(sz) = size {
        wrapper_classes.push(size_class(sz).to_string());
    }
    if loading.get_untracked() {
        wrapper_classes.push("is-loading".to_string());
    }

    let wrapper_class = wrapper_classes.join(" ");

    let name_value = name.get_untracked();
    let is_disabled = disabled.get_untracked();
    let size_attr = list_size.unwrap_or(4).to_string();

    // Initial value snapshot (not reactive) to avoid tachys property binding.
    let initial_values = value.get_untracked();

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    let select_ref: NodeRef<html::Select> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys::{Event, HtmlSelectElement};

        let has_attached = Rc::new(Cell::new(false));
        let select_ref_for_effect = select_ref.clone();
        let update_for_effect = update.clone();

        Effect::new(move |_| {
            if has_attached.get() {
                return;
            }

            let Some(select_element) = select_ref_for_effect.get() else {
                return;
            };

            let update_for_input = update_for_effect.clone();

            let input_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    let target_select = event
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlSelectElement>().ok());

                    let Some(target_select) = target_select else {
                        return;
                    };

                    let selected_options = target_select.selected_options();
                    let mut selected_values = Vec::new();
                    for index in 0..selected_options.length() {
                        if let Some(option) = selected_options.item(index) {
                            selected_values.push(option.value());
                        }
                    }

                    (update_for_input)(selected_values);
                }));

            select_element
                .add_event_listener_with_callback("input", input_closure.as_ref().unchecked_ref())
                .ok();

            has_attached.set(true);
            input_closure.forget();
        });
    }

    view! {
        <div
            class=wrapper_class
            attr:data-testid=data_testid
            attr:data-cy=data_cy
        >
            <select
                node_ref=select_ref
                multiple=true
                size=size_attr
                name=name_value
                disabled=is_disabled
            >
                // Mark initial selected options without reactive bindings.
                {children()}
            </select>
        </div>
    }
}
