/*!
Example page: Tile

AI Pair Programming Notes:
- Demonstrates Bulma Tile layout with ancestor/parent/child structure.
*/

use lbc::prelude::{Tile, TileCtx, TileSize};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn TilePage() -> impl IntoView {
    view! {
        <div class="block">
            <h3 class="title is-5">"Tile"</h3>

            <Tile ctx=TileCtx::Ancestor>
                <Tile ctx=TileCtx::Parent size=TileSize::Four>
                    <Tile ctx=TileCtx::Child tag="article" classes="box">
                        <p class="title is-6">"One"</p>
                        <p>"First tile content."</p>
                    </Tile>
                </Tile>
                <Tile ctx=TileCtx::Parent size=TileSize::Four>
                    <Tile ctx=TileCtx::Child tag="article" classes="box">
                        <p class="title is-6">"Two"</p>
                        <p>"Second tile content."</p>
                    </Tile>
                </Tile>
            </Tile>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lbc::prelude::{Tile, TileCtx, TileSize};
    use leptos::prelude::RenderHtml;

    #[test]
    fn tile_structure_has_classes() {
        let html = view! {
            <Tile ctx=TileCtx::Ancestor>
                <Tile ctx=TileCtx::Parent size=TileSize::Four>
                    <Tile ctx=TileCtx::Child tag="article" classes="box">""</Tile>
                </Tile>
            </Tile>
        }
        .to_html();
        assert!(
            html.contains(r#"class="tile is-ancestor""#),
            "expected ancestor class"
        );
        assert!(
            html.contains("tile is-child box"),
            "expected child box class"
        );
        assert!(html.contains("is-4"), "expected size class on parent tile");
    }
}
