use lbc::prelude::{Block, Content, Control, HeaderSize, Subtitle, Tag, TagColor, Title};
use leptos::prelude::event_target_value;
use leptos::prelude::*;

/// Example page showcasing the Control form component.
pub fn FormControlPage() -> impl IntoView {
    let (typed_text, set_typed_text) = signal(String::new());
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
                        class="input"
                        type="text"
                        placeholder="Type something"
                        on:input=move |ev| set_typed_text.set(event_target_value(&ev))
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
