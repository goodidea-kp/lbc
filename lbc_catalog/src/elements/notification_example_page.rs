use lbc::prelude::{Block, Content, HeaderSize, Notification, Title};
use leptos::prelude::{IntoView, component, view};

#[component]
pub fn NotificationPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Notification"</Title>
            <Content>
                <Notification>
                    {"Plain notification for neutral messaging."}
                </Notification>
                <Notification classes="is-primary">
                    {"Primary notification for important actions."}
                </Notification>
                <Notification classes="is-success is-light">
                    {"Success notification with light styling."}
                </Notification>
            </Content>
        </Block>
    }
}
