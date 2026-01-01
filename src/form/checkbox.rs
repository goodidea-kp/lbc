use crate::util::TestAttr;
use leptos::callback::Callback;
use leptos::prelude::Callable;
use leptos::prelude::*;
#[allow(unused_imports)]
use std::cell::Cell;
#[allow(unused_imports)]
use std::rc::Rc;

/// The 2-state checkbox in its native Bulma format.
///
/// https://bulma.io/documentation/form/checkbox/
///
/// Controlled component:
/// - `checked` is the current value (supports static bool or reactive signal).
/// - `update` is an optional callback invoked with the next value when the user clicks.
///
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
    update: Option<Callback<bool>>,

    /// Additional CSS classes to append to Bulma's "checkbox".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Disable this component.
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Optional test attribute (renders as data-* attribute) on the outer <label>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

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

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <label
            class=class
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            <input
                type="checkbox"
                name=name.clone()
                // Bind reactively so UI reflects changes
                prop:checked=move || checked.get()
                // Bind disabled as an attribute so SSR renders it and it's still reactive at runtime
                disabled=move || disabled.get()
                // Notify parent about user interaction
                on:change=move |ev| {
                    if let Some(cb) = &update {
                        cb.run(event_target_checked(&ev));
                    }
                }
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

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn checkbox_renders_test_attr_as_data_testid() {
        let html = view! {
            <Checkbox name="agree" checked=true test_attr=TestAttr::test_id("checkbox-test")>"Agree"</Checkbox>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="checkbox-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn checkbox_no_test_attr_when_not_provided() {
        let html = view! {
            <Checkbox name="agree" checked=true>"Agree"</Checkbox>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute; got: {}",
            html
        );
    }
}
