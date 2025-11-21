use leptos::prelude::{component, view, IntoView, IntoAny, ClassAttribute, ElementChild, Get, Set, AddAnyAttr, OnAttribute, create_signal, set_timeout};
use leptos::context::provide_context;
use lbc::prelude::{Block, Content, HeaderSize, Notification, Title, Button, Modal, ModalCard, ModalCloserContext};

#[component]
pub fn ModalPage() -> impl IntoView {
    let closer = leptos::prelude::RwSignal::new(String::new());
    provide_context::<ModalCloserContext>(closer);
    let (show_toast, set_show_toast) = create_signal(false);
    view! {
            <Block>
                <Title size=HeaderSize::Is5>"Modal"</Title>
                <Content>
                    <p class="subtitle is-6">"Basic Modal and ModalCard"</p>

                    <Modal
                        id="id2".to_string()
                        classes=""
                        trigger=Box::new(|| view! { <Button color=lbc::elements::button::ButtonColor::Primary>"Open Modal"</Button> }.into_any())
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
                        trigger=Box::new(|| view! { <Button color=lbc::elements::button::ButtonColor::Link>"Open Modal Card"</Button> }.into_any())
                        body=Box::new(|| view! {
                            <div>
                                <p>"Modal card body content."</p>
                            </div>
                        }.into_any())
                        footer=Box::new(move || {
                            let closer = leptos::prelude::use_context::<ModalCloserContext>();
                            let set_show_toast = set_show_toast.clone();
                            view! {
                                <>
                                    <Button
                                        color=lbc::elements::button::ButtonColor::Success
                                        on:click=move |_| {
                                            if let Some(ctx) = closer.clone() {
                                                ctx.set("id1-close".to_string());
                                            }
                                            set_show_toast.set(true);
                                            set_timeout(
                                                move || { set_show_toast.set(false); },
                                                std::time::Duration::from_millis(2000),
                                            );
                                        }
                                    >
                                        "Save changes"
                                    </Button>
                                    <Button
                                        color=lbc::elements::button::ButtonColor::Warning
                                        on:click=move |_| {
                                            if let Some(ctx) = closer.clone() {
                                                ctx.set("id1-close".to_string());
                                            }
                                        }
                                    >
                                        "Cancel"
                                    </Button>
                                </>
                            }.into_any()
                        })
                    />

                    <div class="mt-4"></div>

                    {move || {
                        let closer = leptos::prelude::use_context::<ModalCloserContext>();
                        view! {
                            <Button color=lbc::elements::button::ButtonColor::Danger
                                on:click=move |_| {
                                    if let Some(ctx) = closer.clone() {
                                        ctx.set("id1-close".to_string());
                                    }
                                }
                            >
                                "Close Modal Card via Context"
                            </Button>
                        }.into_any()
                    }}
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
