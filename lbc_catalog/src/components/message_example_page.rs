use lbc::prelude::{
    Block, Button, Buttons, HeaderSize, Message, MessageBody, MessageHeader, Title,
};
use leptos::prelude::{
    AddAnyAttr, ElementChild, Get, IntoAny, IntoView, Set, component,
    view, signal
};

#[component]
pub fn MessagePage() -> impl IntoView {
    let (show_primary, set_show_primary) = signal(true);
    let (color_class, set_color_class) = signal("is-primary".to_string());

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Message"</Title>

            <Buttons>
                <Button on:click=move |_| { set_color_class.set("is-primary".to_string()); set_show_primary.set(true); }>"Primary"</Button>
                <Button classes="is-warning" on:click=move |_| { set_color_class.set("is-warning".to_string()); set_show_primary.set(true); }>"Warning"</Button>
                <Button classes="is-info" on:click=move |_| { set_color_class.set("is-info".to_string()); set_show_primary.set(true); }>"Info"</Button>
                <Button classes="is-light" on:click=move |_| set_show_primary.set(true)>"Show"</Button>
            </Buttons>

            {move || if show_primary.get() {
                view! {
                    <Message
                        classes=color_class.get()
                        closable=true
                        on_close=std::rc::Rc::new(move || set_show_primary.set(false))
                    >
                        <MessageHeader>
                            <p>"Interactive Message"</p>
                        </MessageHeader>
                        <MessageBody>
                            <p>
                                "This is a Bulma-styled message block. Use the buttons above to switch "
                                "color variants or close the message."
                            </p>
                        </MessageBody>
                    </Message>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </Block>
    }
}
