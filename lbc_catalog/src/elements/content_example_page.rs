/*!
Example page: Content

AI Pair Programming Notes:
- Demonstrates Bulma Content element with various tags and classes.
*/

use lbc::prelude::{Block, Title, HeaderSize, Content};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn ContentPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Content"</Title>

            <Content>
                <Title size=HeaderSize::Is6>"Default (div)"</Title>
                <p>"This uses the default div tag."</p>
            </Content>

            <div class="mt-3"></div>

            <Content tag="article" classes="is-small">
                <Title size=HeaderSize::Is6>"Article Content"</Title>
                <p>"This content is wrapped in an article tag with an extra class."</p>
                <ul>
                    <li>"Item one"</li>
                    <li>"Item two"</li>
                </ul>
            </Content>
        </Block>
    }
}
