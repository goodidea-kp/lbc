/*!
Leptos version of Bulma Level components.

- Level: multi-purpose horizontal level container
- LevelLeft: container for level elements grouped to the left
- LevelRight: container for level elements grouped to the right
- LevelItem: individual element of a level container

Follows existing crate patterns:
- optional props via #[prop(optional)]
- classes as Option<Signal<String>>
- customizable HTML tag via tag prop
*/

use leptos::prelude::{
    AnyView, Children, ClassAttribute, CustomAttribute, ElementChild, Get, GetUntracked, IntoAny,
    Signal, component, view,
};

use crate::util::TestAttr;

/// A multi-purpose horizontal level, which can contain almost any other element.
///
/// https://bulma.io/documentation/layout/level/
#[component]
pub fn Level(
    #[prop(optional, into)] classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (nav, div, section, header, footer)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute)
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    children: Children,
) -> AnyView {
    // Build class attribute: "level [extra classes]"
    let mut class_attr = String::from("level");

    if let Some(extra) = classes {
        let extra_val = extra.get_untracked();
        if !extra_val.trim().is_empty() {
            class_attr.push(' ');
            class_attr.push_str(extra_val.trim());
        }
    }

    let tag_name = tag
        .as_ref()
        .map(|t| t.get().to_lowercase())
        .unwrap_or_else(|| "nav".to_string());

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    let node: AnyView = match tag_name.as_str() {
        "div" => view! {
            <div
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </div>
        }
        .into_any(),
        "section" => view! {
            <section
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </section>
        }
        .into_any(),
        "header" => view! {
            <header
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </header>
        }
        .into_any(),
        "footer" => view! {
            <footer
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </footer>
        }
        .into_any(),
        _ => view! {
            <nav
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </nav>
        }
        .into_any(),
    };
    node
}

/// A container for level elements to be grouped to the left of the container.
///
/// https://bulma.io/documentation/layout/level/
#[component]
pub fn LevelLeft(
    #[prop(optional, into)] classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (div, section, nav)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute)
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    children: Children,
) -> AnyView {
    // Build class attribute: "level-left [extra classes]"
    let mut class_attr = String::from("level-left");

    if let Some(extra) = classes {
        let extra_val = extra.get_untracked();
        if !extra_val.trim().is_empty() {
            class_attr.push(' ');
            class_attr.push_str(extra_val.trim());
        }
    }

    let tag_name = tag
        .as_ref()
        .map(|t| t.get().to_lowercase())
        .unwrap_or_else(|| "div".to_string());

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    let node: AnyView = match tag_name.as_str() {
        "section" => view! {
            <section
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </section>
        }
        .into_any(),
        "nav" => view! {
            <nav
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </nav>
        }
        .into_any(),
        _ => view! {
            <div
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </div>
        }
        .into_any(),
    };
    node
}

/// A container for level elements to be grouped to the right of the container.
///
/// https://bulma.io/documentation/layout/level/
#[component]
pub fn LevelRight(
    #[prop(optional, into)] classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (div, section, nav)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute)
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    children: Children,
) -> AnyView {
    // Build class attribute: "level-right [extra classes]"
    let mut class_attr = String::from("level-right");

    if let Some(extra) = classes {
        let extra_val = extra.get_untracked();
        if !extra_val.trim().is_empty() {
            class_attr.push(' ');
            class_attr.push_str(extra_val.trim());
        }
    }

    let tag_name = tag
        .as_ref()
        .map(|t| t.get().to_lowercase())
        .unwrap_or_else(|| "div".to_string());

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    let node: AnyView = match tag_name.as_str() {
        "section" => view! {
            <section
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </section>
        }
        .into_any(),
        "nav" => view! {
            <nav
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </nav>
        }
        .into_any(),
        _ => view! {
            <div
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </div>
        }
        .into_any(),
    };
    node
}

/// An individual element of a level container.
///
/// https://bulma.io/documentation/layout/level/
#[component]
pub fn LevelItem(
    #[prop(optional, into)] classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (div, p, a, span)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute)
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    children: Children,
) -> AnyView {
    // Build class attribute: "level-item [extra classes]"
    let mut class_attr = String::from("level-item");

    if let Some(extra) = classes {
        let extra_val = extra.get_untracked();
        if !extra_val.trim().is_empty() {
            class_attr.push(' ');
            class_attr.push_str(extra_val.trim());
        }
    }

    let tag_name = tag
        .as_ref()
        .map(|t| t.get().to_lowercase())
        .unwrap_or_else(|| "div".to_string());

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    let node: AnyView = match tag_name.as_str() {
        "p" => view! {
            <p
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </p>
        }
        .into_any(),
        "a" => view! {
            <a
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </a>
        }
        .into_any(),
        "span" => view! {
            <span
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </span>
        }
        .into_any(),
        _ => view! {
            <div
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </div>
        }
        .into_any(),
    };
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn level_renders_default() {
        let html = view! { <Level>"X"</Level> }.to_html();
        assert!(
            html.contains(r#"class="level""#),
            "expected base 'level' class, got: {}",
            html
        );
        assert!(
            html.contains("<nav"),
            "expected default nav tag, got: {}",
            html
        );
    }

    #[test]
    fn level_with_custom_tag_and_classes() {
        let html = view! { <Level tag="div" classes="custom-class">"X"</Level> }.to_html();
        assert!(
            html.contains(r#"class="level custom-class""#),
            "expected level with custom class, got: {}",
            html
        );
        assert!(html.contains("<div"), "expected div tag, got: {}", html);
    }

    #[test]
    fn level_left_renders() {
        let html = view! { <LevelLeft>"Left"</LevelLeft> }.to_html();
        assert!(
            html.contains(r#"class="level-left""#),
            "expected level-left class, got: {}",
            html
        );
        assert!(
            html.contains("<div"),
            "expected default div tag, got: {}",
            html
        );
    }

    #[test]
    fn level_right_renders() {
        let html = view! { <LevelRight>"Right"</LevelRight> }.to_html();
        assert!(
            html.contains(r#"class="level-right""#),
            "expected level-right class, got: {}",
            html
        );
    }

    #[test]
    fn level_item_renders() {
        let html = view! { <LevelItem>"Item"</LevelItem> }.to_html();
        assert!(
            html.contains(r#"class="level-item""#),
            "expected level-item class, got: {}",
            html
        );
    }

    #[test]
    fn level_item_with_custom_tag() {
        let html = view! { <LevelItem tag="p">"Paragraph Item"</LevelItem> }.to_html();
        assert!(
            html.contains("<p") && html.contains("level-item"),
            "expected p tag with level-item class, got: {}",
            html
        );
    }

    #[test]
    fn level_complete_structure() {
        let html = view! {
            <Level>
                <LevelLeft>
                    <LevelItem>"Left Item"</LevelItem>
                </LevelLeft>
                <LevelRight>
                    <LevelItem>"Right Item"</LevelItem>
                </LevelRight>
            </Level>
        }
        .to_html();
        assert!(
            html.contains(r#"class="level""#),
            "expected level class, got: {}",
            html
        );
        assert!(
            html.contains("level-left") && html.contains("level-right"),
            "expected level-left and level-right, got: {}",
            html
        );
        assert!(
            html.contains("level-item"),
            "expected level-item, got: {}",
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
    fn level_renders_test_attr_as_data_testid() {
        let html = view! {
            <Level classes="custom" test_attr=TestAttr::test_id("level-test")>
                "X"
            </Level>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="level-test""#),
            "expected data-testid attribute on Level; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn level_no_test_attr_when_not_provided() {
        let html = view! {
            <Level classes="custom">
                "X"
            </Level>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute on Level when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn level_item_renders_test_attr_as_data_testid() {
        let html = view! {
            <LevelItem classes="custom" tag="p" test_attr=TestAttr::test_id("level-item-test")>
                "Item"
            </LevelItem>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="level-item-test""#),
            "expected data-testid attribute on LevelItem; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn level_item_no_test_attr_when_not_provided() {
        let html = view! {
            <LevelItem classes="custom" tag="p">
                "Item"
            </LevelItem>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute on LevelItem when not provided; got: {}",
            html
        );
    }
}
