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
    AnyView, Children, ClassAttribute, ElementChild, Get, GetUntracked, IntoAny, Signal, component, view,
};

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

    let node: AnyView = match tag_name.as_str() {
        "article" => view! { <article class=class_attr.clone()>{children()}</article> }.into_any(),
        "section" => view! { <section class=class_attr.clone()>{children()}</section> }.into_any(),
        "nav" => view! { <nav class=class_attr.clone()>{children()}</nav> }.into_any(),
        "span" => view! { <span class=class_attr.clone()>{children()}</span> }.into_any(),
        _ => view! { <div class=class_attr.clone()>{children()}</div> }.into_any(),
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
