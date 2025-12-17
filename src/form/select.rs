use std::sync::Arc;

#[allow(unused_imports)]
use leptos::prelude::Effect;
#[allow(unused_imports)]
use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, Get, GetUntracked, IntoView, Signal,
    component, view,
};
#[allow(unused_imports)]
use std::cell::Cell;
#[allow(unused_imports)]
use std::rc::Rc;

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

    view! {
        <div
            class=wrapper_class
            attr:data-testid=data_testid
            attr:data-cy=data_cy
        >
            <select
                name=name_value
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
#[component]
pub fn MultiSelect(
    /// The `name` attribute for this form element.
    #[prop(into)]
    name: Signal<String>,

    /// The controlled values of this form element.
    #[prop(into)]
    value: Signal<Vec<String>>,

    /// The callback to be used for propagating changes to this form element's value.
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

    view! {
        <div
            class=wrapper_class
            attr:data-testid=data_testid
            attr:data-cy=data_cy
        >
            <select
                multiple=true
                size=size_attr
                name=name_value
                disabled=is_disabled
            >
                {children()}
            </select>
        </div>
    }
}
