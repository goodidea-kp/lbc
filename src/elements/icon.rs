/*!
Leptos version of Bulma Icon element.

Bulma docs: https://bulma.io/documentation/elements/icon/
*/

use leptos::prelude::{
    Children, ClassAttribute, ElementChild, GetUntracked, IntoView, Signal, component, view,
};

use crate::util::Size;

/// Horizontal alignment for icons, typically used within form controls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconAlignment {
    Left,
    Right,
}

impl IconAlignment {
    fn bulma(self) -> &'static str {
        match self {
            IconAlignment::Left => "is-left",
            IconAlignment::Right => "is-right",
        }
    }
}

/// A container for any type of icon font.
#[component]
pub fn Icon(
    /// Additional CSS classes to append to the base "icon" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// The size of this component; to help prevent page "jumps" during load.
    #[prop(optional)]
    size: Option<Size>,
    /// The alignment of this icon, often used within form controls.
    #[prop(optional)]
    alignment: Option<IconAlignment>,
    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
    /// Child content to render inside the icon
    children: Children,
) -> impl IntoView {
    // Build class attribute: "icon [size/alignment/extra classes]"
    let mut class_parts: Vec<String> = vec!["icon".to_string()];

    if let Some(sz) = size {
        let s = sz.bulma();
        if !s.is_empty() {
            class_parts.push(s.to_string());
        }
    }

    if let Some(align) = alignment {
        class_parts.push(align.bulma().to_string());
    }

    if let Some(extra) = classes {
        let extra_val = extra.get_untracked();
        if !extra_val.trim().is_empty() {
            class_parts.push(extra_val.trim().to_string());
        }
    }

    let class_attr = class_parts.join(" ");

    view! { <span class=class_attr data-testid=test_id>{children()}</span> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn icon_renders_default() {
        let html = view! { <Icon><i class="fa"></i></Icon> }.to_html();
        assert!(
            html.contains(r#"class="icon""#),
            "expected base 'icon' class, got: {}",
            html
        );
        assert!(html.contains("<span"), "expected span tag, got: {}", html);
    }

    #[test]
    fn icon_with_size_alignment_and_extra_classes() {
        let html = view! { <Icon size=Size::Small alignment=IconAlignment::Left classes="has-text-danger"><i class="fa fa-x"></i></Icon> }.to_html();
        assert!(
            html.contains(r#"class="icon is-small is-left has-text-danger""#),
            "expected combined classes, got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn icon_renders_test_id() {
        let html = view! {
            <Icon test_id="icon-test"><i class="fa"></i></Icon>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="icon-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn icon_no_test_id_when_not_provided() {
        let html = view! {
            <Icon><i class="fa"></i></Icon>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute; got: {}",
            html
        );
    }
}
