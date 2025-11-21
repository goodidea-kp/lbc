/*!
Example page: Block

AI Pair Programming Notes:
- Demonstrates Bulma Block element for vertical spacing/sectioning.
*/

use lbc::prelude::{Block, Title, HeaderSize};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn BlockPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Block"</Title>

            <Block>
                <p>"This is a simple block. Use it to separate content vertically."</p>
            </Block>

            <Block classes="has-background-light p-3">
                <p class="is-size-6">"Block with extra classes"</p>
                <p class="has-text-grey">"Adds background and padding for emphasis."</p>
            </Block>

            <Block>
                <p>"Another block to show spacing."</p>
            </Block>
        </Block>
    }
}
