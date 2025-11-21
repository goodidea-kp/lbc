/*!
Leptos version of Bulma Footer component.

- Footer: wraps content in a Bulma "footer" element

Follows existing crate patterns:
- optional props via #[prop(optional)] / #[prop(optional, into)]
- classes as Option<Signal<String>>
*/

use leptos::prelude::{
    Children, ClassAttribute, ElementChild, Get, GetUntracked, IntoView, Signal, component, view,
};

/// A simple responsive footer which can include anything.
///
/// https://bulma.io/documentation/layout/footer/
#[component]
pub fn Footer(
    #[prop(optional, into)] classes: Option<Signal<String>>,
    children: Children,
) -> impl IntoView {
    // Build class attribute: "footer [extra classes]"
    let mut class_attr = String::from("footer");

    if let Some(extra) = classes {
        let extra_val = extra.get_untracked();
        if !extra_val.trim().is_empty() {
            class_attr.push(' ');
            class_attr.push_str(extra_val.trim());
        }
    }

    view! {
        <footer class=class_attr>
            {children()}
        </footer>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn footer_renders_default() {
        let html = view! { <Footer>"X"</Footer> }.to_html();
        assert!(
            html.contains(r#"class="footer""#),
            "expected base 'footer' class, got: {}",
            html
        );
        assert!(html.contains('X'));
    }

    #[test]
    fn footer_with_extra_classes() {
        let html =
            view! { <Footer classes="has-background-dark has-text-white">"Y"</Footer> }.to_html();
        assert!(
            html.contains(r#"class="footer has-background-dark has-text-white""#),
            "expected combined classes, got: {}",
            html
        );
    }
}
