use std::sync::Arc;

use leptos::prelude::*;

/// The 2-state checkbox in its native Bulma format.
///
/// https://bulma.io/documentation/form/checkbox/
///
/// Controlled component:
/// - `checked` is the current value (supports static bool or reactive signal).
/// - `update` is an optional callback invoked with the next value when the user clicks.
#[component]
pub fn Checkbox(
    /// The `name` attribute for this form element.
    #[prop(into)]
    name: String,

    /// The controlled value of this form element.
    ///
    /// Accepts a bool or a reactive signal.
    #[prop(into)]
    checked: Signal<bool>,

    /// Optional callback to propagate changes to the parent with the new value.
    #[prop(optional)]
    update: Option<Arc<dyn Fn(bool) + Send + Sync>>,

    /// Additional CSS classes to append to Bulma's "checkbox".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Disable this component.
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Label/content shown next to the checkbox.
    children: Children,
) -> impl IntoView {
    let class = move || {
        let extra = classes.get();
        if extra.trim().is_empty() {
            "checkbox".to_string()
        } else {
            format!("checkbox {}", extra)
        }
    };

    let on_click = {
        let checked = checked.clone();
        let update = update.clone();
        move |_| {
            if let Some(handler) = update.as_ref() {
                let next = !checked.get();
                (handler)(next);
            }
        }
    };

    view! {
        <label class=class>
            <input
                type="checkbox"
                name=name.clone()
                prop:checked=checked
                on:click=on_click
                disabled=disabled
            />
            {children()}
        </label>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn checkbox_renders_base_class_and_label_text() {
        let html = view! { <Checkbox name="agree" checked=true>{"Agree"}</Checkbox> }.to_html();
        assert!(
            html.contains(r#"class="checkbox""#),
            "expected 'checkbox' class in: {}",
            html
        );
        assert!(html.contains(">Agree<"), "expected label text in: {}", html);
    }

    #[test]
    fn checkbox_sets_name_attribute() {
        let html = view! { <Checkbox name="terms" checked=false>{"X"}</Checkbox> }.to_html();
        assert!(
            html.contains(r#"name="terms""#),
            "expected name attribute in: {}",
            html
        );
    }

    #[test]
    fn checkbox_allows_extra_classes() {
        let html =
            view! { <Checkbox name="c" checked=true classes="is-small custom">"X"</Checkbox> }
                .to_html();
        assert!(
            html.contains(r#"class="checkbox is-small custom""#),
            "expected merged classes in: {}",
            html
        );
    }

    #[test]
    fn checkbox_can_be_disabled() {
        let html =
            view! { <Checkbox name="d" checked=false disabled=true>"X"</Checkbox> }.to_html();
        assert!(
            html.contains(r#"disabled"#),
            "expected disabled attribute on input in: {}",
            html
        );
    }
}
