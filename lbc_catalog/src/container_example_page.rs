/*!
Example page: Container

AI Pair Programming Notes:
- Minimal example of Container with the fluid prop.
*/

use lbc::prelude::{Block, Title, HeaderSize, Notification, Container};
use leptos::prelude::{IntoView, component, view};

#[component]
pub fn ContainerPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Container"</Title>
            <Container fluid=true>
                <Notification classes="is-link">"This is a fluid container"</Notification>
            </Container>
        </Block>
    }
}
