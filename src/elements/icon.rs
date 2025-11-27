/*!
Leptos version of Bulma Icon element.

Bulma docs: https://bulma.io/documentation/elements/icon/
*/

use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, GetUntracked, IntoView, Signal,
    component, view,
};

use crate::util::{Size, TestAttr};

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
    /// Optional test attribute (renders as data-* attribute) on the root <span>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
    /// Child content to render inside the icon
    children: Children,
) -> impl IntoView {
    // Build class attribute: "icon [size/alignment/extra classes]"
    let mut class_parts: Vec<String> = vec!["icon".to_string()];

    if let Some(size_value) = size {
        let size_class = size_value.bulma();
        if !size_class.is_empty() {
            class_parts.push(size_class.to_string());
        }
    }

    if let Some(alignment_value) = alignment {
        class_parts.push(alignment_value.bulma().to_string());
    }

    if let Some(extra) = classes {
        let extra_val = extra.get_untracked();
        if !extra_val.trim().is_empty() {
            class_parts.push(extra_val.trim().to_string());
        }
    }

    let class_attr = class_parts.join(" ");

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <span
            class=class_attr
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {children()}
        </span>
    }
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
        let html = view! {
            <Icon
                size=Size::Small
                alignment=IconAlignment::Left
                classes="has-text-danger"
            >
                <i class="fa fa-x"></i>
            </Icon>
        }
        .to_html();
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
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn icon_renders_test_id() {
        let html = view! {
            <Icon test_attr=TestAttr::test_id("icon-test")><i class="fa"></i></Icon>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="icon-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn icon_no_test_attr_when_not_provided() {
        let html = view! {
            <Icon><i class="fa"></i></Icon>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn icon_accepts_custom_test_attr_key() {
        let html = view! {
            <Icon test_attr=TestAttr::new("data-cy", "icon-cy")><i class="fa"></i></Icon>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="icon-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
