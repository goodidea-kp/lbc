/*!
Example page: Box

AI Pair Programming Notes:
- Demonstrates Bulma Box element for content container.
*/

use lbc::prelude::{Block, Title, HeaderSize, Box};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn BoxPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Box"</Title>

            <Box>
                <p>"This is a simple box. Use it to group related content."</p>
            </Box>

            <div class="mt-3"></div>

            <Box classes="has-background-light p-4">
                <p class="is-size-6">"Box with extra classes"</p>
                <p class="has-text-grey">"Adds background and padding for emphasis."</p>
            </Box>
        </Block>
    }
}
