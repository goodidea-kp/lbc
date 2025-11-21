use leptos::html;
use leptos::prelude::{
    Children, ClassAttribute, ElementChild, Get, IntoView, NodeRef, NodeRefAttribute, OnAttribute,
    PropAttribute, Signal, component, event_target_value, view,
};
use std::sync::Arc;

use crate::util::Size;

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
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let loading = loading.clone();
        move || {
            let mut parts = vec!["select".to_string()];

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

            parts.join(" ")
        }
    };

    let on_input = {
        let update = update.clone();
        move |ev| {
            let new_value = event_target_value(&ev);
            (update)(new_value);
        }
    };

    view! {
        <div class=move || class()>
            <select
                name=name.get()
                prop:value=value.get()
                disabled=disabled.get()
                on:input=on_input
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
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let loading = loading.clone();
        move || {
            let mut parts = vec!["select".to_string(), "is-multiple".to_string()];

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

            parts.join(" ")
        }
    };

    let select_ref: NodeRef<html::Select> = NodeRef::new();

    // Gather all selected option values on input.
    let on_input = {
        let update = update.clone();
        let select_ref = select_ref.clone();
        move |_| {
            if let Some(select) = select_ref.get() {
                let opts = select.selected_options();
                let mut selected_values = Vec::new();
                for index in 0..opts.length() {
                    if let Some(elem) = opts.item(index) {
                        if let Some(val) =
                            elem.get_attribute("value").or_else(|| elem.text_content())
                        {
                            selected_values.push(val);
                        }
                    }
                }
                (update)(selected_values);
            }
        }
    };

    let size_attr = list_size.unwrap_or(4).to_string();
    let joined_value = move || value.get().join(",");

    view! {
        <div class=move || class()>
            <select
                multiple=true
                size=size_attr
                name=name.get()
                prop:value=joined_value()
                disabled=disabled.get()
                on:input=on_input
                node_ref=select_ref
            >
                {children()}
            </select>
        </div>
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

    fn noop_vec() -> Arc<dyn Fn(Vec<String>) + Send + Sync> {
        Arc::new(|_v| {})
    }

    #[test]
    fn select_renders_wrapper_and_attributes() {
        let html = view! {
            <Select name="kind" value="x" update=noop()>
                <option value="x">"X"</option>
                <option value="y">"Y"</option>
            </Select>
        }
        .to_html();

        assert!(
            html.contains(r#"class="select""#),
            "expected Bulma 'select' wrapper class; got: {}",
            html
        );
        assert!(
            html.contains(r#"<select"#),
            "expected select element; got: {}",
            html
        );
        assert!(
            html.contains(r#"name="kind""#),
            "expected name attribute; got: {}",
            html
        );
        assert!(
            html.contains(r#"value="x""#),
            "expected value attribute; got: {}",
            html
        );
    }

    #[test]
    fn select_loading_and_size_classes() {
        let html = view! {
            <Select name="n" value="v" loading=true update=noop()>
                <option value="v">"V"</option>
            </Select>
        }
        .to_html();
        assert!(
            html.contains("is-loading"),
            "expected is-loading class; got: {}",
            html
        );

        let html_small = view! {
            <Select name="n" value="v" size=Size::Small update=noop()>
                <option value="v">"V"</option>
            </Select>
        }
        .to_html();
        assert!(
            html_small.contains("is-small"),
            "expected size class; got: {}",
            html_small
        );
    }

    #[test]
    fn multi_select_renders_multiple_and_size() {
        let html = view! {
            <MultiSelect name="m" value=vec!["a".to_string()] list_size=6 update=noop_vec()>
                <option value="a">"A"</option>
                <option value="b">"B"</option>
            </MultiSelect>
        }
        .to_html();

        assert!(
            html.contains("is-multiple"),
            "expected is-multiple class on wrapper; got: {}",
            html
        );
        assert!(
            html.contains(r#"multiple="true""#) || html.contains(r#"multiple"#),
            "expected multiple attribute on select; got: {}",
            html
        );
        assert!(
            html.contains(r#"size="6""#),
            "expected size attribute; got: {}",
            html
        );
    }
}
