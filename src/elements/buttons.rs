/*!
Leptos version of Bulma Buttons group container.

Bulma docs: https://bulma.io/documentation/elements/button/#list-of-buttons
Supports:
- size: are-small | are-medium | are-large
- has-addons
- alignment: is-centered | is-right
*/

use leptos::prelude::{
    Children, ClassAttribute, ElementChild, Get, IntoView, Signal, component, view,
};

use crate::components::tabs::Alignment;
use crate::util::Size;

fn size_class_for_group(size: Size) -> &'static str {
    match size {
        Size::Small => "are-small",
        Size::Normal => "",
        Size::Medium => "are-medium",
        Size::Large => "are-large",
    }
}

/// A simple wrapper for a group of buttons (`<div class="buttons">`).
#[component]
pub fn Buttons(
    /// Extra classes to apply to the root "buttons" container.
    #[prop(optional, into)]
    classes: Option<Signal<String>>,

    /// Optional size for the whole group (maps to are-small/are-medium/are-large).
    #[prop(optional)]
    size: Option<Size>,

    /// Addons style for grouped buttons.
    #[prop(optional, into)]
    addons: Signal<bool>,

    /// Alignment: Centered or Right (left is default).
    #[prop(optional)]
    alignment: Option<Alignment>,

    /// Group content (Button components or anchors with Bulma button classes).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let addons = addons.clone();
        move || {
            let mut parts: Vec<String> = vec!["buttons".to_string()];

            if let Some(sz) = size {
                let sc = size_class_for_group(sz);
                if !sc.is_empty() {
                    parts.push(sc.to_string());
                }
            }

            if let Some(align) = alignment {
                parts.push(match align {
                    Alignment::Centered => "is-centered".to_string(),
                    Alignment::Right => "is-right".to_string(),
                });
            }

            if addons.get() {
                parts.push("has-addons".to_string());
            }

            if let Some(extra) = &classes {
                let s = extra.get();
                if !s.trim().is_empty() {
                    parts.push(s);
                }
            }

            parts.join(" ")
        }
    };

    view! { <div class=class>{children()}</div> }
}
