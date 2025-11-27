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

            if let Some(size_value) = size {
                let size_class = size_class_for_group(size_value);
                if !size_class.is_empty() {
                    parts.push(size_class.to_string());
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
                let value = extra.get();
                if !value.trim().is_empty() {
                    parts.push(value);
                }
            }

            parts.join(" ")
        }
    };

    view! { <div class=class>{children()}</div> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::tabs::Alignment;
    use crate::util::Size;
    use leptos::prelude::RenderHtml;

    #[test]
    fn buttons_renders_base_class_and_children() {
        let html = view! {
            <Buttons>
                <button class="button">"One"</button>
            </Buttons>
        }
        .to_html();

        assert!(
            html.contains(r#"class="buttons""#),
            "expected base 'buttons' class; got: {}",
            html
        );
        assert!(
            html.contains("One"),
            "expected child content; got: {}",
            html
        );
    }

    #[test]
    fn buttons_applies_size_alignment_and_addons() {
        let html = view! {
            <Buttons size=Size::Small alignment=Alignment::Right addons=true>
                <button class="button">"X"</button>
            </Buttons>
        }
        .to_html();

        assert!(
            html.contains("are-small"),
            "expected size class; got: {}",
            html
        );
        assert!(
            html.contains("is-right"),
            "expected alignment class; got: {}",
            html
        );
        assert!(
            html.contains("has-addons"),
            "expected has-addons class; got: {}",
            html
        );
    }
}
