/*!
Leptos version of Bulma Section component.

- Section: wraps content in a Bulma "section" element
- SectionSize: maps to "is-medium" | "is-large"

Follows existing crate patterns:
- optional props via #[prop(optional)]
- classes as Option<Signal<String>>
*/

use leptos::prelude::{
    Children, ClassAttribute, ElementChild, GetUntracked, IntoView, Signal, component, view,
};

/// The 2 sizes available for sections, which controls spacing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionSize {
    Medium,
    Large,
}

impl SectionSize {
    pub fn bulma(self) -> &'static str {
        match self {
            SectionSize::Medium => "is-medium",
            SectionSize::Large => "is-large",
        }
    }
}

/// A simple container to divide your page into sections.
///
/// https://bulma.io/documentation/layout/section/
#[component]
pub fn Section(
    #[prop(optional)] size: Option<SectionSize>,
    #[prop(optional, into)] classes: Option<Signal<String>>,
    children: Children,
) -> impl IntoView {
    // Build class attribute: "section [is-medium|is-large] [extra classes]"
    let mut class_attr = String::from("section");

    if let Some(size) = size {
        let size_class = size.bulma();
        if !size_class.is_empty() {
            class_attr.push(' ');
            class_attr.push_str(size_class);
        }
    }

    if let Some(extra) = classes {
        let extra_val = extra.get_untracked();
        if !extra_val.trim().is_empty() {
            class_attr.push(' ');
            class_attr.push_str(extra_val.trim());
        }
    }

    view! {
        <section class=class_attr>
            {children()}
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn section_renders_default() {
        let html = view! { <Section>"X"</Section> }.to_html();
        assert!(
            html.contains(r#"class="section""#),
            "expected base 'section' class, got: {}",
            html
        );
    }

    #[test]
    fn section_renders_medium() {
        let html = view! { <Section size=SectionSize::Medium>"X"</Section> }.to_html();
        assert!(
            html.contains(r#"class="section is-medium""#),
            "expected 'section is-medium', got: {}",
            html
        );
    }

    #[test]
    fn section_renders_large_with_extra_classes() {
        let html =
            view! { <Section size=SectionSize::Large classes="custom cls">"X"</Section> }.to_html();
        assert!(
            html.contains(r#"class="section is-large custom cls""#),
            "expected combined classes, got: {}",
            html
        );
    }
}
