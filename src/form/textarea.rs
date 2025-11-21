use leptos::prelude::{
    ClassAttribute, ElementChild, Get, GlobalAttributes, IntoAny, IntoView, OnAttribute,
    PropAttribute, Signal, StyleAttribute, component, event_target_value, view,
};
use std::sync::Arc;

use crate::elements::icon::Icon;
use crate::util::Size;

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

    // input handler is defined inline at the usage sites to avoid moving captures

    // Render an optional "GenAI ribbon" icon overlay if requested.
    move || {
        if is_genai.get() {
            let update_ai = update.clone();
            view! {
                <div id="context" style="position:relative">
                    <Icon size=Size::Small classes="is-pulled-right ribbon">
                        <i class="fa-brands fa-openai"></i>
                    </Icon>
                    <textarea
                        name=name.get()
                        prop:value=value.get()
                        on:input=move |ev| {
                            let new_value = event_target_value(&ev);
                            (update_ai)(new_value);
                        }
                        class=move || class()
                        placeholder=placeholder.get()
                        disabled=disabled.get()
                        readonly=readonly.get()
                        rows=rows.unwrap_or(0).to_string()
                    />
                </div>
            }
            .into_any()
        } else {
            let update_plain = update.clone();
            view! {
                <textarea
                    name=name.get()
                    prop:value=value.get()
                    on:input=move |ev| {
                        let new_value = event_target_value(&ev);
                        (update_plain)(new_value);
                    }
                    class=move || class()
                    placeholder=placeholder.get()
                    disabled=disabled.get()
                    readonly=readonly.get()
                    rows=rows.unwrap_or(0).to_string()
                />
            }
            .into_any()
        }
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
        let html = view! {
            <TextArea name="g" value="" is_genai=true update=noop() />
        }
        .to_html();
        assert!(
            html.contains("ribbon"),
            "expected ribbon icon when is_genai; got: {}",
            html
        );
    }
}
