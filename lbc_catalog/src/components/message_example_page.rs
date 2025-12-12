use std::cell::Cell;
use std::rc::Rc;

use lbc::prelude::{Block, Buttons, HeaderSize, Message, MessageBody, MessageHeader, Title};
use leptos::html;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, IntoAny, IntoView, NodeRef, NodeRefAttribute, Set,
    component, signal, view,
};

#[component]
pub fn MessagePage() -> impl IntoView {
    let (show_primary, set_show_primary) = signal(true);
    let (color_class, set_color_class) = signal("is-primary".to_string());

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually on wasm32.
    let primary_button_ref: NodeRef<html::Button> = NodeRef::new();
    let warning_button_ref: NodeRef<html::Button> = NodeRef::new();
    let info_button_ref: NodeRef<html::Button> = NodeRef::new();
    let show_button_ref: NodeRef<html::Button> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::Event;

        fn attach_button_click_once(
            button_ref: NodeRef<html::Button>,
            has_attached: Rc<Cell<bool>>,
            on_click: Rc<dyn Fn()>,
        ) {
            Effect::new(move |_| {
                if has_attached.get() {
                    return;
                }

                let Some(button_element) = button_ref.get() else {
                    return;
                };

                let on_click_for_event = on_click.clone();
                let click_closure: Closure<dyn FnMut(Event)> =
                    Closure::wrap(Box::new(move |event: Event| {
                        event.prevent_default();
                        (on_click_for_event)();
                    }));

                button_element
                    .add_event_listener_with_callback(
                        "click",
                        click_closure.as_ref().unchecked_ref(),
                    )
                    .ok();

                has_attached.set(true);
                click_closure.forget();
            });
        }

        attach_button_click_once(
            primary_button_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_color_class = set_color_class.clone();
                let set_show_primary = set_show_primary.clone();
                move || {
                    set_color_class.set("is-primary".to_string());
                    set_show_primary.set(true);
                }
            }),
        );

        attach_button_click_once(
            warning_button_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_color_class = set_color_class.clone();
                let set_show_primary = set_show_primary.clone();
                move || {
                    set_color_class.set("is-warning".to_string());
                    set_show_primary.set(true);
                }
            }),
        );

        attach_button_click_once(
            info_button_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_color_class = set_color_class.clone();
                let set_show_primary = set_show_primary.clone();
                move || {
                    set_color_class.set("is-info".to_string());
                    set_show_primary.set(true);
                }
            }),
        );

        attach_button_click_once(
            show_button_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_show_primary = set_show_primary.clone();
                move || set_show_primary.set(true)
            }),
        );
    }

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Message"</Title>

            <Buttons>
                <button
                    node_ref=primary_button_ref
                    class="button is-primary"
                    type="button"
                >
                    "Primary"
                </button>
                <button
                    node_ref=warning_button_ref
                    class="button is-warning"
                    type="button"
                >
                    "Warning"
                </button>
                <button
                    node_ref=info_button_ref
                    class="button is-info"
                    type="button"
                >
                    "Info"
                </button>
                <button
                    node_ref=show_button_ref
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
