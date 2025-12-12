use std::time::Duration;

use lbc::prelude::{
    Block, Button, Content, HeaderSize, Modal, ModalCard, ModalCloserContext, Notification, Title,
};
use leptos::context::provide_context;
use leptos::html;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, IntoAny, IntoView, NodeRef, NodeRefAttribute, Set,
    component, set_timeout, signal, view,
};

#[cfg(target_arch = "wasm32")]
use std::cell::Cell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;

#[component]
pub fn ModalPage() -> impl IntoView {
    let closer = leptos::prelude::RwSignal::new(String::new());
    provide_context::<ModalCloserContext>(closer);

    let (show_toast, set_show_toast) = signal(false);

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually on wasm32.
    let save_button_ref: NodeRef<html::Button> = NodeRef::new();
    let cancel_button_ref: NodeRef<html::Button> = NodeRef::new();
    let close_via_context_button_ref: NodeRef<html::Button> = NodeRef::new();

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
            save_button_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let closer = closer.clone();
                let set_show_toast = set_show_toast.clone();
                move || {
                    closer.set("id1-close".to_string());
                    set_show_toast.set(true);
                    set_timeout(
                        {
                            let set_show_toast = set_show_toast.clone();
                            move || set_show_toast.set(false)
                        },
                        Duration::from_millis(2000),
                    );
                }
            }),
        );

        attach_button_click_once(
            cancel_button_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let closer = closer.clone();
                move || {
                    closer.set("id1-close".to_string());
                }
            }),
        );

        attach_button_click_once(
            close_via_context_button_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let closer = closer.clone();
                move || {
                    closer.set("id1-close".to_string());
                }
            }),
        );
    }

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Modal"</Title>
            <Content>
                <p class="subtitle is-6">"Basic Modal and ModalCard"</p>

                <Modal
                    id="id2".to_string()
                    classes=""
                    trigger=Box::new(|| view! {
                        <Button color=lbc::elements::button::ButtonColor::Primary>
                            "Open Modal"
                        </Button>
                    }.into_any())
                >
                    <lbc::prelude::Box>
                        <p>"This is a simple modal content."</p>
                        <p class="mt-2">"Click outside or the close button to dismiss."</p>
                    </lbc::prelude::Box>
                </Modal>

                <div class="mt-4"></div>

                <ModalCard
                    id="id1".to_string()
                    title="Modal Card".to_string()
                    classes=""
                    trigger=Box::new(|| view! {
                        <Button color=lbc::elements::button::ButtonColor::Link>
                            "Open Modal Card"
                        </Button>
                    }.into_any())
                    body=Box::new(|| view! {
                        <div>
                            <p>"Modal card body content."</p>
                        </div>
                    }.into_any())
                    footer=Box::new(move || {
                        view! {
                            <>
                                <button
                                    node_ref=save_button_ref
                                    class="button is-success"
                                    type="button"
                                >
                                    "Save changes"
                                </button>

                                <button
                                    node_ref=cancel_button_ref
                                    class="button is-warning"
                                    type="button"
                                >
                                    "Cancel"
                                </button>
                            </>
                        }.into_any()
                    })
                />

                <div class="mt-4"></div>

                <button
                    node_ref=close_via_context_button_ref
                    class="button is-danger"
                    type="button"
                >
                    "Close Modal Card via Context"
                </button>

                {move || {
                    if show_toast.get() {
                        view! { <Notification classes="is-success mt-3">"Saved successfully."</Notification> }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
            </Content>
        </Block>
    }
}
