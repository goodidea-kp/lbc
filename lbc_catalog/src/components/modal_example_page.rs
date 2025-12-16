use lbc::prelude::{
    Block, Button, Content, HeaderSize, Modal, ModalCard, ModalCloserContext, Notification, Title,
};
use leptos::context::provide_context;
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, IntoAny, IntoView, component, signal, view,
};

#[component]
pub fn ModalPage() -> impl IntoView {
    let closer = leptos::prelude::RwSignal::new(String::new());
    provide_context::<ModalCloserContext>(closer);

    let (show_toast, set_show_toast) = signal(false);

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
                                    class="button is-success"
                                    type="button"
                                >
                                    "Save changes"
                                </button>

                                <button
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
