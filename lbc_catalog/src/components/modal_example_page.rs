use leptos::prelude::{OnAttribute, Set};
use lbc::prelude::{
    Block, Button, Content, HeaderSize, Modal, ModalCard, ModalCloserContext, Notification, Title,
};
use leptos::callback::Callback;
use leptos::context::provide_context;
use leptos::prelude::Set;
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, IntoAny, IntoView, component, signal, view,
};

#[component]
pub fn ModalPage() -> impl IntoView {
    let closer = leptos::prelude::RwSignal::new(String::new());
    provide_context::<ModalCloserContext>(closer);
    #[allow(unused)]
    let (show_toast, set_show_toast) = signal(false);
    let trigger2 = Box::new(move || {
        view! {

                        <Button color=lbc::elements::button::ButtonColor::Primary
                            on_click=Callback::new(move |_|closer.set("id2-open".to_string()))
        >
                            "Open Modal"
                        </Button>
                    }
        .into_any()
    });

    let trigger1 = Box::new(move || {
        view! {
            <Button color=lbc::elements::button::ButtonColor::Link
              on_click=Callback::new(move |_|closer.set("id1-open".to_string()))
               >
                "Open Modal Card"
            </Button>
        }
        .into_any()
    });
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Modal"</Title>
            <Content>
                <p class="subtitle is-6">"Basic Modal and ModalCard"</p>
                <Modal
                    id="id2".to_string()
                    classes=""
                    trigger=trigger2
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
                    trigger=trigger1
                    body=Box::new(|| view! {
                        <div>
                            <p>"Modal card body content."</p>
                        </div>
                    }.into_any())
                    footer=Box::new(move || {
                        view! {
                            <>
                                <lbc::prelude::Button
                                    classes="is-success"
                 on_click={Callback::new(move |_|{set_show_toast.set(true);closer.set("id1-close".to_string())})}
                                >
                                    "Save changes"
                                </lbc::prelude::Button>

                                <lbc::prelude::Button
                                    classes="is-warning"
                on_click=Callback::new(move |_|closer.set("id1-close".to_string()))
                                >
                                    "Cancel"
                                </lbc::prelude::Button>
                            </>
                        }.into_any()
                    })
                />

                <div class="mt-4"></div>

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
