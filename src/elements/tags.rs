/*!
Leptos version of Bulma Tags container.

Bulma docs: https://bulma.io/documentation/elements/tag/#list-of-tags
*/

use leptos::prelude::{Children, ClassAttribute, ElementChild, Get, IntoView, Signal, component, view};

/// A simple wrapper for a group of tags (`<div class="tags">`).
#[component]
pub fn Tags(
    /// Extra classes to apply to the root "tags" container.
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// Group content (Tag components or plain elements with Bulma tag classes).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || {
            let mut parts: Vec<String> = vec!["tags".to_string()];
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
