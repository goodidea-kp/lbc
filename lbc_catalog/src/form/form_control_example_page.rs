use lbc::prelude::{Block, Content, Control, HeaderSize, Subtitle, Tag, TagColor, Title};
use leptos::html;
use leptos::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Example page showcasing the Control form component.
#[allow(non_snake_case)]
pub fn FormControlPage() -> impl IntoView {
    let (typed_text, set_typed_text) = signal(String::new());

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:input` and attach the input listener manually on wasm32.
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::{Event, HtmlInputElement};

        let has_attached = Rc::new(Cell::new(false));
        let input_ref_for_effect = input_ref.clone();
        let set_typed_text_for_effect = set_typed_text.clone();

        Effect::new(move |_| {
            if has_attached.get() {
                return;
            }

            let Some(input_element) = input_ref_for_effect.get() else {
                return;
            };

            let input_element: HtmlInputElement = input_element.into();

            let input_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    let target_input = event
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlInputElement>().ok());

                    let Some(target_input) = target_input else {
                        return;
                    };

                    set_typed_text_for_effect.set(target_input.value());
                }));

            input_element
                .add_event_listener_with_callback("input", input_closure.as_ref().unchecked_ref())
                .ok();

            has_attached.set(true);
            input_closure.forget();
        });
    }

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Control"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Control"</Subtitle>
                <Control>
                    <input class="input" type="text" placeholder="Your name"/>
                </Control>

                <div class="mt-4"></div>

                <Subtitle size=HeaderSize::Is6>"Interactive Control (on input)"</Subtitle>
                <Control>
                    <input
                        node_ref=input_ref
                        class="input"
                        type="text"
                        placeholder="Type something"
                    />
                </Control>
                <p class="mt-2">
                    <strong>"You typed: "</strong>
                    { move || typed_text.get() }
                </p>

                <div class="mt-4"></div>

                <Subtitle size=HeaderSize::Is6>"Expanded Control"</Subtitle>
                <Control expanded=true>
                    <input class="input" type="email" placeholder="Email address"/>
                </Control>

                <div class="mt-4"></div>

                <Subtitle size=HeaderSize::Is6>"Custom Tag (article)"</Subtitle>
                <Control tag="article" classes="has-background-light p-3">
                    <Tag color=TagColor::Info>"Wrapped content inside article tag"</Tag>
                </Control>
            </Content>
        </Block>
    }
}
