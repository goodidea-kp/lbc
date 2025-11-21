use leptos::prelude::{
    Children, ClassAttribute, ElementChild, Get, IntoView, OnAttribute, Signal, component, view,
};
use std::sync::Arc;

/// The mutually exclusive radio buttons in their native format.
///
/// https://bulma.io/documentation/form/radio/
///
/// All LBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
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
) -> impl IntoView {
    // Compute Bulma "radio" class plus any extras provided by consumer.
    let class = {
        let classes = classes.clone();
        move || {
            let extra = classes.get().trim().to_string();
            if extra.is_empty() {
                "radio".to_string()
            } else {
                format!("radio {}", extra)
            }
        }
    };

    // Determine whether this radio is currently checked by comparing the group's checked_value with this radio's value.
    let is_checked = move || {
        if let Some(cv) = &checked_value {
            cv == &value.get()
        } else {
            false
        }
    };

    // When user selects this radio, propagate this radio's value to the parent.
    let on_input = {
        let update = update.clone();
        let value = value.clone();
        move |_| {
            (update)(value.get());
        }
    };

    view! {
        <label class=move || class()>
            <input
                type="radio"
                name=name.get()
                value=value.get()
                checked=is_checked()
                on:input=on_input
                disabled=disabled.get()
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
