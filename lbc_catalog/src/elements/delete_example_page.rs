/*!
Example page: Delete

AI Pair Programming Notes:
- Demonstrates Bulma Delete element with default and custom tags.
*/

use lbc::prelude::{Block, Buttons, Delete, HeaderSize, Notification, Title};
use leptos::html;
use leptos::prelude::{
    AddAnyAttr, ClassAttribute, Effect, ElementChild, Get, IntoAny, IntoView, NodeRef,
    NodeRefAttribute, Set, component, set_timeout, signal, view,
};
use std::sync::Arc;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
fn console_log(message: &str) {
    use leptos::wasm_bindgen::JsValue;
    use leptos::web_sys::console;

    console::log_1(&JsValue::from_str(message));
}

#[cfg(not(target_arch = "wasm32"))]
fn console_log(message: &str) {
    println!("{message}");
}

#[component]
pub fn DeletePage() -> impl IntoView {
    let (is_confirming, set_is_confirming) = signal(false);
    let (user_choice, set_user_choice) = signal::<Option<bool>>(None);
    let (show_toast, set_show_toast) = signal(false);

    let prompt_text = "configration to continue for Delete";

    let delete_click = Arc::new(move |_| {
        console_log("[DeletePage] Delete (button) clicked -> show confirmation");
        set_is_confirming.set(true);
        set_user_choice.set(None);
    });

    let anchor_delete_click = Arc::new(move |_| {
        console_log("[DeletePage] Delete (anchor) clicked -> show toast");
        set_show_toast.set(true);
        set_timeout(
            move || {
                console_log("[DeletePage] toast timeout -> hide toast");
                set_show_toast.set(false);
            },
            Duration::from_secs(2),
        );
    });

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually for the confirmation buttons.
    let continue_button_ref: NodeRef<html::Button> = NodeRef::new();
    let cancel_button_ref: NodeRef<html::Button> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::Event;

        // Continue button
        let continue_button_ref_for_effect = continue_button_ref.clone();
        let set_user_choice_for_effect = set_user_choice.clone();
        let set_is_confirming_for_effect = set_is_confirming.clone();

        Effect::new(move |_| {
            let Some(button_element) = continue_button_ref_for_effect.get() else {
                return;
            };

            let click_closure: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_event: Event| {
                console_log("[DeletePage] Continue clicked");
                set_user_choice_for_effect.set(Some(true));
                set_is_confirming_for_effect.set(false);
            }));

            button_element
                .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
                .ok();

            click_closure.forget();
        });

        // Cancel button
        let cancel_button_ref_for_effect = cancel_button_ref.clone();
        let set_user_choice_for_effect = set_user_choice.clone();
        let set_is_confirming_for_effect = set_is_confirming.clone();

        Effect::new(move |_| {
            let Some(button_element) = cancel_button_ref_for_effect.get() else {
                return;
            };

            let click_closure: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_event: Event| {
                console_log("[DeletePage] Cancel clicked");
                set_user_choice_for_effect.set(Some(false));
                set_is_confirming_for_effect.set(false);
            }));

            button_element
                .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
                .ok();

            click_closure.forget();
        });
    }

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
                                <button
                                    node_ref=continue_button_ref
                                    class="button is-success is-light"
                                    type="button"
                                >
                                    "Continue"
                                </button>
                                <button
                                    node_ref=cancel_button_ref
                                    class="button is-danger is-light"
                                    type="button"
                                >
                                    "Cancel"
                                </button>
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
