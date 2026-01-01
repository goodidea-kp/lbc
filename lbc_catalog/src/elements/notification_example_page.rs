use lbc::prelude::{Block, Button, Content, HeaderSize, Notification, Title};
use leptos::callback::Callback;
use leptos::prelude::{AddAnyAttr, IntoView, Set, component, signal, view};

#[component]
#[allow(snake_case)]
pub fn NotificationPage() -> impl IntoView {
    let (open, set_open) = signal(false);

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

                <Title size=HeaderSize::Is5>"Toast Notification"</Title>
                <Button on:click=move |_| set_open.set(true)>"Show Toast"</Button>
                <Notification
                    toast=true
                    open=open
                    set_open=Callback::new(move |val| set_open.set(val))
                    auto_hide_ms=5000
                    classes="is-primary"
                >
                    {"This is a toast notification that will autohide after 5 seconds."}
                </Notification>
            </Content>
        </Block>
    }
}
