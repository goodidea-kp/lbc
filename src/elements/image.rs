/*!
Leptos wrapper for a plain <img> element, following LBC conventions.

Bulma docs for images: https://bulma.io/documentation/elements/image/
This component renders only the <img>. Wrap it with a Bulma "image" figure if needed.
*/

use leptos::prelude::{
    ClassAttribute, Get, GlobalAttributes, IntoView, Signal, StyleAttribute, component, view,
};

/// Simple image element with optional classes and style.
#[component]
pub fn Image(
    /// The image source URL.
    #[prop(into)]
    src: Signal<String>,

    /// Alternative text for accessibility.
    #[prop(default = "".to_string().into(), into)]
    alt: Signal<String>,

    /// Additional CSS classes to apply to the <img>.
    #[prop(optional, into)]
    classes: Option<Signal<String>>,

    /// Inline style, if needed.
    #[prop(optional, into)]
    style: Option<Signal<String>>,
) -> impl IntoView {
    let class_attr = {
        let classes = classes.clone();
        move || {
            if let Some(c) = &classes {
                let v = c.get();
                if v.trim().is_empty() {
                    String::new()
                } else {
                    v
                }
            } else {
                String::new()
            }
        }
    };

    let style_attr = {
        let style = style.clone();
        move || {
            if let Some(s) = &style {
                s.get()
            } else {
                String::new()
            }
        }
    };

    view! {
        <img
            src=src.get()
            alt=alt.get()
            class=class_attr
            style=style_attr
        />
    }
}
