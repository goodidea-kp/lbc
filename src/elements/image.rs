/*!
Leptos wrapper for a plain <img> element, following LBC conventions.

Bulma docs for images: https://bulma.io/documentation/elements/image/
This component renders only the <img>. Wrap it with a Bulma "image" figure if needed.
*/

use leptos::prelude::{
    ClassAttribute, CustomAttribute, Get, IntoView, Signal, StyleAttribute, component, view,
};

use crate::util::TestAttr;

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

    /// Optional test attribute (renders as data-* attribute) on the <img>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (for example, `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let class_attr = {
        let classes = classes.clone();
        move || {
            if let Some(class_signal) = &classes {
                let value = class_signal.get();
                if value.trim().is_empty() {
                    String::new()
                } else {
                    value
                }
            } else {
                String::new()
            }
        }
    };

    let style_attr = {
        let style = style.clone();
        move || {
            if let Some(style_signal) = &style {
                style_signal.get()
            } else {
                String::new()
            }
        }
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <img
            src=src.get()
            alt=alt.get()
            class=class_attr
            style=style_attr
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn image_renders_basic_img() {
        let html = view! { <Image src="x.png" alt="alt text" /> }.to_html();
        assert!(html.contains(r#"<img"#), "expected img tag; got: {}", html);
        assert!(
            html.contains(r#"src="x.png""#),
            "expected src attribute; got: {}",
            html
        );
        assert!(
            html.contains(r#"alt="alt text""#),
            "expected alt attribute; got: {}",
            html
        );
    }

    #[test]
    fn image_applies_classes_and_style() {
        let html = view! {
            <Image
                src="x.png"
                alt=""
                classes=Signal::derive(|| "has-shadow".to_string())
                style=Signal::derive(|| "border-radius:4px".to_string())
            />
        }
        .to_html();

        assert!(
            html.contains(r#"class="has-shadow""#),
            "expected class attribute; got: {}",
            html
        );
        // Accept both with and without trailing semicolon, depending on renderer
        assert!(
            html.contains(r#"style="border-radius:4px""#)
                || html.contains(r#"style="border-radius:4px;""#),
            "expected style attribute; got: {}",
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
    fn image_renders_test_id() {
        let html = view! {
            <Image src="x.png" alt="alt" test_attr=TestAttr::test_id("image-test") />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="image-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn image_no_test_attr_when_not_provided() {
        let html = view! {
            <Image src="x.png" alt="alt" />
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn image_accepts_custom_test_attr_key() {
        let html = view! {
            <Image src="x.png" alt="alt" test_attr=TestAttr::new("data-cy", "image-cy") />
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="image-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
