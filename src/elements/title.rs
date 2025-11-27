/*!
Leptos version of Bulma Title and Subtitle elements.

Bulma docs: https://bulma.io/documentation/elements/title/
*/

use leptos::prelude::{
    AnyView, Children, ClassAttribute, CustomAttribute, ElementChild, Get, IntoAny, Signal,
    component, view,
};

use crate::util::TestAttr;

/// The six sizes available for titles & subtitles.
///
/// https://bulma.io/documentation/elements/title/#sizes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderSize {
    Is1,
    Is2,
    Is3,
    Is4,
    Is5,
    Is6,
}

impl HeaderSize {
    pub fn bulma(self) -> &'static str {
        match self {
            HeaderSize::Is1 => "is-1",
            HeaderSize::Is2 => "is-2",
            HeaderSize::Is3 => "is-3",
            HeaderSize::Is4 => "is-4",
            HeaderSize::Is5 => "is-5",
            HeaderSize::Is6 => "is-6",
        }
    }
}

/// A simple heading to add depth to your page.
///
/// https://bulma.io/documentation/elements/title/
#[component]
pub fn Title(
    /// Additional CSS classes to append to the base "title" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (h1-h6, p, div, span)
    #[prop(default = "h3".to_string().into(), into)]
    tag: Signal<String>,
    /// Maintain the normal spacing between titles and subtitles.
    #[prop(optional)]
    is_spaced: bool,
    /// The size of this component.
    #[prop(optional)]
    size: Option<HeaderSize>,
    /// Optional test attribute (renders as data-* attribute) on the rendered element.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (for example, `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
    /// Child content to render inside the title
    children: Children,
) -> AnyView {
    let class_str = move || {
        let mut parts = vec!["title"];

        if is_spaced {
            parts.push("is-spaced");
        }

        let mut result = parts.join(" ");

        if let Some(size_val) = size {
            result.push(' ');
            result.push_str(size_val.bulma());
        }

        if let Some(extra) = &classes {
            let extra_val = extra.get();
            if !extra_val.trim().is_empty() {
                result.push(' ');
                result.push_str(extra_val.trim());
            }
        }

        result
    };

    let tag_name = tag.get().to_lowercase();

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    match tag_name.as_str() {
        "h1" => view! {
            <h1
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h1>
        }
        .into_any(),
        "h2" => view! {
            <h2
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h2>
        }
        .into_any(),
        "h3" => view! {
            <h3
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h3>
        }
        .into_any(),
        "h4" => view! {
            <h4
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h4>
        }
        .into_any(),
        "h5" => view! {
            <h5
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h5>
        }
        .into_any(),
        "h6" => view! {
            <h6
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h6>
        }
        .into_any(),
        "p" => view! {
            <p
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </p>
        }
        .into_any(),
        "div" => view! {
            <div
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </div>
        }
        .into_any(),
        "span" => view! {
            <span
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </span>
        }
        .into_any(),
        _ => view! {
            <h3
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h3>
        }
        .into_any(),
    }
}

/// A simple subtitle to add depth to your page.
///
/// https://bulma.io/documentation/elements/title/
#[component]
pub fn Subtitle(
    /// Additional CSS classes to append to the base "subtitle" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (h1-h6, p, div, span)
    #[prop(default = "h3".to_string().into(), into)]
    tag: Signal<String>,
    /// The size of this component.
    #[prop(optional)]
    size: Option<HeaderSize>,
    /// Optional test attribute (renders as data-* attribute) on the rendered element.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (for example, `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
    /// Child content to render inside the subtitle
    children: Children,
) -> AnyView {
    let class_str = move || {
        let mut result = String::from("subtitle");

        if let Some(size_val) = size {
            result.push(' ');
            result.push_str(size_val.bulma());
        }

        if let Some(extra) = &classes {
            let extra_val = extra.get();
            if !extra_val.trim().is_empty() {
                result.push(' ');
                result.push_str(extra_val.trim());
            }
        }

        result
    };

    let tag_name = tag.get().to_lowercase();

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    match tag_name.as_str() {
        "h1" => view! {
            <h1
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h1>
        }
        .into_any(),
        "h2" => view! {
            <h2
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h2>
        }
        .into_any(),
        "h3" => view! {
            <h3
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h3>
        }
        .into_any(),
        "h4" => view! {
            <h4
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h4>
        }
        .into_any(),
        "h5" => view! {
            <h5
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h5>
        }
        .into_any(),
        "h6" => view! {
            <h6
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h6>
        }
        .into_any(),
        "p" => view! {
            <p
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </p>
        }
        .into_any(),
        "div" => view! {
            <div
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </div>
        }
        .into_any(),
        "span" => view! {
            <span
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </span>
        }
        .into_any(),
        _ => view! {
            <h3
                class=class_str()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </h3>
        }
        .into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn title_renders_default() {
        let html = view! { <Title>"Hello World"</Title> }.to_html();
        assert!(
            html.contains(r#"class="title""#),
            "expected base 'title' class, got: {}",
            html
        );
        assert!(
            html.contains("<h3"),
            "expected default h3 tag, got: {}",
            html
        );
        assert!(html.contains("Hello World"), "expected child text");
    }

    #[test]
    fn title_with_size() {
        let html = view! { <Title size=HeaderSize::Is1>"Big Title"</Title> }.to_html();
        assert!(
            html.contains(r#"class="title is-1""#),
            "expected title with is-1 size, got: {}",
            html
        );
    }

    #[test]
    fn title_with_custom_tag() {
        let html = view! { <Title tag="h1">"Custom Tag"</Title> }.to_html();
        assert!(html.contains("<h1"), "expected h1 tag, got: {}", html);
        assert!(
            html.contains(r#"class="title""#),
            "expected title class, got: {}",
            html
        );
    }

    #[test]
    fn title_is_spaced() {
        let html = view! { <Title is_spaced=true>"Spaced Title"</Title> }.to_html();
        assert!(
            html.contains("is-spaced"),
            "expected is-spaced class, got: {}",
            html
        );
    }

    #[test]
    fn title_with_custom_classes() {
        let html = view! { <Title classes="has-text-centered">"Centered"</Title> }.to_html();
        assert!(
            html.contains(r#"class="title has-text-centered""#),
            "expected custom classes, got: {}",
            html
        );
    }

    #[test]
    fn title_all_options() {
        let html = view! {
            <Title tag="h2" size=HeaderSize::Is3 is_spaced=true classes="custom">
                "Full Featured"
            </Title>
        }
        .to_html();
        assert!(html.contains("<h2"), "expected h2 tag");
        assert!(html.contains("is-3"), "expected is-3 size");
        assert!(html.contains("is-spaced"), "expected is-spaced");
        assert!(html.contains("custom"), "expected custom class");
    }

    #[test]
    fn subtitle_renders_default() {
        let html = view! { <Subtitle>"Subtitle Text"</Subtitle> }.to_html();
        assert!(
            html.contains(r#"class="subtitle""#),
            "expected base 'subtitle' class, got: {}",
            html
        );
        assert!(
            html.contains("<h3"),
            "expected default h3 tag, got: {}",
            html
        );
        assert!(html.contains("Subtitle Text"), "expected child text");
    }

    #[test]
    fn subtitle_with_size() {
        let html = view! { <Subtitle size=HeaderSize::Is4>"Sized Subtitle"</Subtitle> }.to_html();
        assert!(
            html.contains(r#"class="subtitle is-4""#),
            "expected subtitle with is-4 size, got: {}",
            html
        );
    }

    #[test]
    fn subtitle_with_custom_tag_and_classes() {
        let html = view! {
            <Subtitle tag="h5" classes="has-text-grey">
                "Custom Subtitle"
            </Subtitle>
        }
        .to_html();
        assert!(html.contains("<h5"), "expected h5 tag, got: {}", html);
        assert!(
            html.contains("has-text-grey"),
            "expected custom class, got: {}",
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
    fn title_renders_test_attr_as_data_testid() {
        let html = view! {
            <Title test_attr=TestAttr::test_id("title-test")>"Title"</Title>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="title-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn title_no_test_attr_when_not_provided() {
        let html = view! {
            <Title>"Title"</Title>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn subtitle_renders_test_attr_as_data_testid() {
        let html = view! {
            <Subtitle test_attr=TestAttr::test_id("subtitle-test")>"Subtitle"</Subtitle>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="subtitle-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn subtitle_no_test_attr_when_not_provided() {
        let html = view! {
            <Subtitle>"Subtitle"</Subtitle>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn title_accepts_custom_test_attr_key() {
        let html = view! {
            <Title test_attr=TestAttr::new("data-cy", "title-cy")>"Title"</Title>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="title-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn subtitle_accepts_custom_test_attr_key() {
        let html = view! {
            <Subtitle test_attr=TestAttr::new("data-cy", "subtitle-cy")>"Subtitle"</Subtitle>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="subtitle-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
