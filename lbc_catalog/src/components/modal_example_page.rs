use lbc::prelude::{
    Block, Button, Content, HeaderSize, Modal, ModalCard, ModalControllerContext,
    ModalControllerProvider, Notification, Title,
};
use leptos::callback::Callback;
use leptos::prelude::Set;
use leptos::prelude::{
    component, signal, view, ClassAttribute, ElementChild, Get, IntoAny, IntoView,
};

#[component]
#[allow(non_snake_case)]
pub fn ModalPage() -> impl IntoView {
    // Provide the controller context locally for this page so it doesn't depend on
    // the catalog app root being wrapped in <ModalControllerProvider>.
    view! {
        <ModalControllerProvider>
            <ModalPageInner />
        </ModalControllerProvider>
    }
}

#[component]
#[allow(non_snake_case)]
fn ModalPageInner() -> impl IntoView {
    let controller = leptos::prelude::use_context::<ModalControllerContext>().expect(
        "ModalControllerContext not found. This should be provided by <ModalControllerProvider>.",
    );

    #[allow(unused)]
    let (show_toast, set_show_toast) = signal(false);

    let trigger2 = {
        let controller = controller.clone();
        Box::new(move || {
            view! {
                <Button
                    color=lbc::elements::button::ButtonColor::Primary
                    on_click=Callback::new(move |_| controller.open("id2"))
                >
                    "Open Modal"
                </Button>
            }
            .into_any()
        })
    };

    let trigger1 = {
        let controller = controller.clone();
        Box::new(move || {
            view! {
                <Button
                    color=lbc::elements::button::ButtonColor::Link
                    on_click=Callback::new(move |_| controller.open("id1"))
                >
                    "Open Modal Card"
                </Button>
            }
            .into_any()
        })
    };

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
                    footer={
                        let controller = controller.clone();
                        Box::new(move || {
                            let controller_save = controller.clone();
                            let controller_cancel = controller.clone();
                            view! {
                                <>
                                    <lbc::prelude::Button
                                        classes="is-success"
                                        on_click={
                                            Callback::new(move |_| {
                                                set_show_toast.set(true);
                                                controller_save.close("id1");
                                            })
                                        }
                                    >
                                        "Save changes"
                                    </lbc::prelude::Button>

                                    <lbc::prelude::Button
                                        classes="is-warning"
                                        on_click=Callback::new(move |_| controller_cancel.close("id1"))
                                    >
                                        "Cancel"
                                    </lbc::prelude::Button>
                                </>
                            }
                            .into_any()
                        })
                    }
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
