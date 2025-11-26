/*!
Leptos version of Bulma Tile component.

- Tile: renders a Bulma "tile" element with optional context, vertical, and size modifiers
- TileCtx: "is-ancestor" | "is-parent" | "is-child"
- TileSize: "is-1" .. "is-12"

Follows existing crate patterns:
- optional props via #[prop(optional)]
- classes as Option<Signal<String>>
*/

use leptos::prelude::{
    AnyView, Children, ClassAttribute, CustomAttribute, ElementChild, Get, GetUntracked, IntoAny,
    Signal, component, view,
};

use crate::util::TestAttr;

/// Tile context modifiers.
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileCtx {
    Ancestor,
    Parent,
    Child,
}

impl TileCtx {
    fn bulma(self) -> &'static str {
        match self {
            TileCtx::Ancestor => "is-ancestor",
            TileCtx::Parent => "is-parent",
            TileCtx::Child => "is-child",
        }
    }
}

/// Tile size modifiers.
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileSize {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl TileSize {
    fn bulma(self) -> &'static str {
        match self {
            TileSize::One => "is-1",
            TileSize::Two => "is-2",
            TileSize::Three => "is-3",
            TileSize::Four => "is-4",
            TileSize::Five => "is-5",
            TileSize::Six => "is-6",
            TileSize::Seven => "is-7",
            TileSize::Eight => "is-8",
            TileSize::Nine => "is-9",
            TileSize::Ten => "is-10",
            TileSize::Eleven => "is-11",
            TileSize::Twelve => "is-12",
        }
    }
}

/// A single tile element to build 2-dimensional layouts.
///
/// https://bulma.io/documentation/layout/tiles/
#[component]
pub fn Tile(
    #[prop(optional)] ctx: Option<TileCtx>,
    #[prop(optional)] vertical: bool,
    #[prop(optional)] size: Option<TileSize>,
    #[prop(optional, into)] classes: Option<Signal<String>>,
    /// The HTML tag to use for this component (div, article, section, nav, span)
    #[prop(optional, into)]
    tag: Option<Signal<String>>,

    /// Optional test attribute for the root element.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (e.g., `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    children: Children,
) -> AnyView {
    // Build class attribute: "tile [ctx] [is-vertical] [is-N] [extra classes]"
    let mut class_attr = String::from("tile");

    if let Some(ctx) = ctx {
        class_attr.push(' ');
        class_attr.push_str(ctx.bulma());
    }

    if vertical {
        class_attr.push_str(" is-vertical");
    }

    if let Some(size) = size {
        class_attr.push(' ');
        class_attr.push_str(size.bulma());
    }

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

    // Derive specific optional attributes that our macro can render.
    // This mirrors the Pagination implementation: we support a small, explicit set of keys.
    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    let node: AnyView = match tag_name.as_str() {
        "article" => view! {
            <article
                class=class_attr.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </article>
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
    fn tile_default_renders_base_class() {
        let html = view! { <Tile>"X"</Tile> }.to_html();
        assert!(
            html.contains(r#"class="tile""#),
            "expected base 'tile' class, got: {}",
            html
        );
    }

    #[test]
    fn tile_parent_vertical_size_classes() {
        let html = view! { <Tile ctx=TileCtx::Parent vertical=true size=TileSize::Four>"X"</Tile> }
            .to_html();
        assert!(
            html.contains(r#"class="tile is-parent is-vertical is-4""#),
            "expected combined classes, got: {}",
            html
        );
    }

    #[test]
    fn tile_child_article_box() {
        let html =
            view! { <Tile ctx=TileCtx::Child tag="article" classes="box">"X"</Tile> }.to_html();
        assert!(
            html.contains("<article"),
            "expected article tag, got: {}",
            html
        );
        assert!(
            html.contains("tile is-child box"),
            "expected child and box classes, got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::{Size, TestAttr};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn tile_renders_test_attr_as_data_testid() {
        let html = view! {
            <Tile
                ctx=TileCtx::Parent
                vertical=true
                size=TileSize::Four
                classes="box"
                test_attr="tile-test"
            >
                "X"
            </Tile>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="tile-test""#),
            "expected data-testid attribute on Tile; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn tile_no_test_attr_when_not_provided() {
        let html = view! {
            <Tile ctx=TileCtx::Parent vertical=true size=TileSize::Four classes="box">
                "X"
            </Tile>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on Tile when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn tile_accepts_custom_test_attr_key() {
        let html = view! {
            <Tile
                ctx=TileCtx::Parent
                vertical=true
                size=TileSize::Four
                classes="box"
                test_attr=TestAttr::new("data-cy", "tile-cy")
            >
                "X"
            </Tile>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="tile-cy""#),
            "expected custom data-cy attribute on Tile; got: {}",
            html
        );
    }
}
