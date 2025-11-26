/*!
Leptos wrapper for a plain <img> element, following LBC conventions.

Bulma docs for images: https://bulma.io/documentation/elements/image/
This component renders only the <img>. Wrap it with a Bulma "image" figure if needed.
*/

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, Get, IntoView, Signal, StyleAttribute, component, view};

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

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
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
            data-testid=test_id
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
        assert!(
            html.contains(r#"<img"#),
            "expected img tag; got: {}",
            html
        );
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
        assert!(
            html.contains(r#"style="border-radius:4px""#),
            "expected style attribute; got: {}",
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
    fn image_renders_test_id() {
        let html = view! {
            <Image src="x.png" alt="alt" test_id="image-test" />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="image-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn image_no_test_id_when_not_provided() {
        let html = view! {
            <Image src="x.png" alt="alt" />
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute; got: {}",
            html
        );
    }
}
