use lbc::prelude::{Block, Buttons, HeaderSize, Message, MessageBody, MessageHeader, Title};
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, IntoAny, IntoView, Set, component, signal, view,
};

#[component]
pub fn MessagePage() -> impl IntoView {
    let (show_primary, set_show_primary) = signal(true);
    let (color_class, set_color_class) = signal("is-primary".to_string());

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Message"</Title>

            <Buttons>
                <button
                    class="button is-primary"
                    type="button"
                >
                    "Primary"
                </button>
                <button
                    class="button is-warning"
                    type="button"
                >
                    "Warning"
                </button>
                <button
                    class="button is-info"
                    type="button"
                >
                    "Info"
                </button>
                <button
                    class="button is-light"
                    type="button"
                >
                    "Show"
                </button>
            </Buttons>

            {move || if show_primary.get() {
                view! {
                    <Message
                        classes=color_class.get()
                        closable=true
                        on_close=std::rc::Rc::new({
                            let set_show_primary = set_show_primary.clone();
                            move || set_show_primary.set(false)
                        })
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
