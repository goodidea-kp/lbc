/*!
Example page: Delete

AI Pair Programming Notes:
- Demonstrates Bulma Delete element with default and custom tags.
*/

use lbc::prelude::{Block, Button, Buttons, Delete, HeaderSize, Notification, Title};
use leptos::prelude::AddAnyAttr;
use leptos::prelude::{
    Callback, ClassAttribute, ElementChild, Get, IntoAny, IntoView, Set, component, set_timeout,
    signal, view,
};
use std::time::Duration;

#[component]
#[allow(non_snake_case)]
pub fn DeletePage() -> impl IntoView {
    let (is_confirming, set_is_confirming) = signal(false);
    let (user_choice, set_user_choice) = signal::<Option<bool>>(None);
    let (show_toast, set_show_toast) = signal(false);

    let prompt_text = "configration to continue for Delete";

    let delete_click = Callback::new(move |_| {
        set_is_confirming.set(true);
        set_user_choice.set(None);
    });

    let anchor_delete_click = Callback::new(move |_| {
        set_show_toast.set(true);
        set_timeout(move || set_show_toast.set(false), Duration::from_secs(2));
    });

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Delete"</Title>

            <p class="mb-2">"Default delete (button tag):"</p>
            <Delete on_click=delete_click.clone() />

            {move || {
                if is_confirming.get() {
                    view! {
                        <Notification classes="is-warning mt-3">
                            <p>{prompt_text}</p>
                            <Buttons classes="mt-2">
                                <Button
                                    classes="button is-success is-light"
                                    on:click=move |_| {
                                        set_user_choice.set(Some(true));
                                        set_is_confirming.set(false);
                                    }
                                >
                                    "Continue"
                                </Button>
                                <Button
                                    classes="button is-danger is-light"
                                    on:click=move |_| {
                                        set_user_choice.set(Some(false));
                                        set_is_confirming.set(false);
                                    }
                                >
                                    "Cancel"
                                </Button>
                            </Buttons>
                        </Notification>
                    }
                    .into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}

            {move || match user_choice.get() {
                Some(true) => view! {
                    <Notification classes="is-success mt-3">
                        "Proceeding: configuration accepted for Delete."
                    </Notification>
                }
                .into_any(),
                Some(false) => view! {
                    <Notification classes="is-danger mt-3">
                        "Stopped: configuration rejected for Delete."
                    </Notification>
                }
                .into_any(),
                None => view! { <></> }.into_any(),
            }}

            <div class="mt-3"></div>

            <p class="mb-2">"Anchor tag with extra classes:"</p>
            <Delete tag="a" classes="is-large" on_click=anchor_delete_click.clone() />

            {move || {
                if show_toast.get() {
                    view! {
                        <Notification classes="is-success mt-3">
                            "Successfully deleted!"
                        </Notification>
                    }
                    .into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}

            <div class="mt-3"></div>

            <p class="mb-2">"Delete with child content (not typical in Bulma, but supported):"</p>
            <Delete classes="mr-2">"×"</Delete>
        </Block>
    }
}
