use std::sync::Arc;

use leptos::html;
#[allow(unused_imports)]
use leptos::prelude::Effect;
#[allow(unused_imports)]
use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, Get, GetUntracked, IntoView, NodeRef,
    NodeRefAttribute, OnAttribute, PropAttribute, Signal, component, event_target,
    event_target_value, view,
};
// Note: avoid deep web-sys feature usage to keep this component lightweight and portable.

use crate::util::{Size, TestAttr};

// Browser-only helpers for MultiSelect
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{EventTarget, HtmlOptionElement, HtmlSelectElement};

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

    // Change handler: extract selected value and propagate via update
    let on_change = {
        let update = update.clone();
        move |ev| {
            // For single select, browser ensures exactly one selected value (unless empty option)
            let new_value = event_target_value(&ev);
            (update)(new_value);
        }
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
                // Bind the value to keep the element controlled and in sync
                prop:value=value
                on:change=on_change
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

    // Change handler: collect all selected option values and propagate
    #[cfg(target_arch = "wasm32")]
    fn extract_selected_values(select: &HtmlSelectElement) -> Vec<String> {
        // Prefer selected_options() to avoid iterating all options when possible
        let collection = select.selected_options();
        let len = collection.length();
        let mut out = Vec::with_capacity(len as usize);
        for i in 0..len {
            if let Some(el) = collection.item(i) {
                if let Ok(opt) = el.dyn_into::<HtmlOptionElement>() {
                    out.push(opt.value());
                }
            }
        }
        out
    }

    // Split handler definition per target to satisfy type inference in trunk/wasm builds
    #[cfg(target_arch = "wasm32")]
    let on_change_multi = {
        let update = update.clone();
        move |ev: web_sys::Event| {
            if let Some(target) = ev
                .target()
                .and_then(|t: EventTarget| t.dyn_into::<HtmlSelectElement>().ok())
            {
                let values = extract_selected_values(&target);
                (update)(values);
                return;
            }
            // Fallback: no target or cast failed
            let v = event_target_value(&ev);
            let out = if v.is_empty() { Vec::new() } else { vec![v] };
            (update)(out);
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let on_change_multi = {
        let update = update.clone();
        move |ev| {
            // SSR/tests path: provide a minimal, stable fallback
            let v = event_target_value(&ev);
            let out = if v.is_empty() { Vec::new() } else { vec![v] };
            (update)(out);
        }
    };

    // Keep a reference to the <select> so we can mirror the controlled Vec<String>
    // into the DOM-selected state of <option> elements at runtime (browser only).
    let select_ref: NodeRef<html::Select> = NodeRef::new();

    // Synchronize DOM selection with the controlled `value` whenever it changes (browser only).
    #[cfg(target_arch = "wasm32")]
    {
        use std::collections::HashSet;

        let select_ref = select_ref.clone();
        let value = value.clone();
        Effect::new(move |_| {
            if let Some(sel) = select_ref.get() {
                let current = value.get(); // reactive read
                let set: HashSet<String> = current.into_iter().collect();

                // Iterate options and set their selected state
                let options = sel.options();
                let len = options.length();
                for i in 0..len {
                    if let Some(node) = options.item(i) {
                        if let Ok(opt) = node.dyn_into::<HtmlOptionElement>() {
                            let v = opt.value();
                            opt.set_selected(set.contains(&v));
                        }
                    }
                }
            }
        });
    }

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
                node_ref=select_ref
                on:change=on_change_multi
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

    fn noop_s() -> Arc<dyn Fn(String) + Send + Sync + 'static> {
        Arc::new(|_| {})
    }
    fn noop_m() -> Arc<dyn Fn(Vec<String>) + Send + Sync + 'static> {
        Arc::new(|_| {})
    }

    #[test]
    fn select_renders_with_value_and_name() {
        let html = view! {
            <Select name="pet" value="cat" update=noop_s()>
                <option value="cat">"Cat"</option>
                <option value="dog">"Dog"</option>
            </Select>
        }
        .to_html();
        assert!(
            html.contains(r#"class="select""#),
            "expected wrapper class; got: {}",
            html
        );
        assert!(
            html.contains(r#"name="pet""#),
            "expected name attribute; got: {}",
            html
        );
    }

    #[test]
    fn multi_select_renders_multiple_and_size() {
        let html = view! {
            <MultiSelect name="fruits" value=vec!["a".to_string()] update=noop_m() list_size=6>
                <option value="a">"A"</option>
                <option value="b">"B"</option>
            </MultiSelect>
        }
        .to_html();
        assert!(
            html.contains("is-multiple"),
            "expected is-multiple class; got: {}",
            html
        );
        assert!(
            html.contains("<select multiple"),
            "expected boolean multiple attr; got: {}",
            html
        );
        assert!(
            html.contains(r#"size="6""#),
            "expected size attr; got: {}",
            html
        );
    }
}
