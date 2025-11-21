use leptos::children::Children;
use leptos::ev::MouseEvent;
use leptos::prelude::{ClassAttribute, ElementChild, Get, OnAttribute, Signal};
use leptos::{IntoView, component, view};

use crate::util::Size;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonColor {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Dark,
    Light,
    Black,
    White,
    Text,
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

    view! {
        <button
            class=class
            disabled=move || disabled.get()
            on:click=move |event| {
                if let Some(cb) = on_click_callback.as_ref() {
                    (cb)(event);
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
