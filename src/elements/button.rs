use crate::util::{Size, TestAttr};
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::ev::MouseEvent;
#[allow(unused_imports)]
use leptos::prelude::Effect;
use leptos::prelude::{
    Callable, ClassAttribute, CustomAttribute, ElementChild, Get, IntoView, OnAttribute, Signal,
    component, view,
};
#[allow(unused_imports)]
use std::cell::Cell;
#[allow(unused_imports)]
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonColor {
    Black,
    Danger,
    Dark,
    Info,
    Light,
    Link,
    Primary,
    Success,
    Text,
    Warning,
    White,
}
impl ButtonColor {
    fn bulma(self) -> &'static str {
        match self {
            ButtonColor::Primary => "is-primary",
            ButtonColor::Link => "is-link",
            ButtonColor::Info => "is-info",
            ButtonColor::Success => "is-success",
            ButtonColor::Warning => "is-warning",
            ButtonColor::Danger => "is-danger",
            ButtonColor::Dark => "is-dark",
            ButtonColor::Light => "is-light",
            ButtonColor::Black => "is-black",
            ButtonColor::White => "is-white",
            ButtonColor::Text => "is-text",
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional)] color: Option<ButtonColor>,
    #[prop(optional)] size: Option<Size>,
    #[prop(optional)] outlined: bool,
    #[prop(optional)] inverted: bool,
    #[prop(optional)] light: bool,
    #[prop(optional, into)] loading: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] classes: Option<Signal<String>>,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
    /// Optional test attribute (renders as data-* attribute)
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
    children: Children,
) -> impl IntoView {
    let class = move || {
        let mut class_parts: Vec<&str> = vec!["button"];
        if let Some(color_value) = color {
            class_parts.push(color_value.bulma());
        }
        if let Some(size_value) = size {
            let size_class = size_value.bulma();
            if !size_class.is_empty() {
                class_parts.push(size_class);
            }
        }
        if outlined {
            class_parts.push("is-outlined");
        }
        if inverted {
            class_parts.push("is-inverted");
        }
        if light {
            class_parts.push("is-light");
        }
        if loading.get() {
            class_parts.push("is-loading");
        }
        if let Some(class_signal) = &classes {
            let extra_classes = class_signal.get();
            if !extra_classes.is_empty() {
                return format!("{} {}", class_parts.join(" "), extra_classes);
            }
        }
        class_parts.join(" ")
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <button
            class=class
            disabled=move || disabled.get()
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    // gloo_console::log!("Button clicked");
                    cb.run(ev);
                }
            }
        >
            {children()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn button_renders_primary() {
        let html = view! { <Button color=ButtonColor::Primary>"Save"</Button> }.to_html();
        assert!(
            html.contains(r#"class="button is-primary""#),
            "expected Bulma primary button class"
        );
        assert!(
            html.contains(">Save<"),
            "expected button label to be rendered"
        );
    }

    #[test]
    fn button_loading_and_size() {
        let html = view! { <Button loading=true size=Size::Small>"Loading"</Button> }.to_html();
        assert!(
            html.contains("button is-small is-loading"),
            "expected size and loading classes"
        );
    }

    #[test]
    fn button_renders_test_id() {
        let html = view! {
            <Button test_attr=TestAttr::test_id("test-button")>"Content"</Button>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="test-button""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[test]
    fn button_no_test_id_when_not_provided() {
        let html = view! { <Button>"Content"</Button> }.to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[test]
    fn button_accepts_custom_test_attr_key() {
        let html = view! {
            <Button test_attr=TestAttr::new("data-cy", "button-cy")>"Content"</Button>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="button-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn button_click_callback() {
        let clicked = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let clicked_clone = clicked.clone();

        let cb = Callback::new(move |_| {
            clicked_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        });

        cb.run(MouseEvent::new("click").unwrap());

        assert!(clicked.load(std::sync::atomic::Ordering::SeqCst));
    }
}
