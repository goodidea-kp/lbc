use leptos::children::Children;
use leptos::ev::MouseEvent;
use leptos::html;
use leptos::prelude::{
    ClassAttribute, CustomAttribute, Effect, ElementChild, Get, IntoView, NodeRef, NodeRefAttribute,
    Signal, component, view,
};

use crate::util::{Size, TestAttr};

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
    #[prop(optional)] on_click: Option<std::rc::Rc<dyn Fn(MouseEvent)>>,
    /// Optional test attribute (renders as data-* attribute)
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
    children: Children,
) -> impl IntoView {
    let on_click_callback = on_click.clone();

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

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach the click listener manually on wasm32.
    let button_ref: NodeRef<html::Button> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys::Event;

        let button_ref_for_effect = button_ref.clone();
        let on_click_for_effect = on_click_callback.clone();

        Effect::new(move |_| {
            let Some(button_element) = button_ref_for_effect.get() else {
                return;
            };

            // If no callback was provided, don't attach a listener.
            let Some(on_click_callback) = on_click_for_effect.clone() else {
                return;
            };

            let click_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    // Convert the DOM event into Leptos' MouseEvent type.
                    // If conversion fails, we just skip calling the callback.
                    let Ok(mouse_event) = event.dyn_into::<MouseEvent>() else {
                        return;
                    };
                    (on_click_callback)(mouse_event);
                }));

            button_element
                .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
                .ok();

            // Keep closure alive for the lifetime of the page/app.
            click_closure.forget();
        });
    }

    view! {
        <button
            node_ref=button_ref
            class=class
            disabled=move || disabled.get()
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
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
    fn button_disabled_flag() {
        let html = view! { <Button disabled=true>"X"</Button> }.to_html();
        // Some SSR renderers may render boolean attributes as `disabled` or `disabled=""`
        assert!(
            html.contains(r#"class="button""#) && html.contains("disabled"),
            "expected disabled attribute on button, got: {}",
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

    #[wasm_bindgen_test]
    fn button_no_test_id_when_not_provided() {
        let html = view! { <Button>"Content"</Button> }.to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
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
