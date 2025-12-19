use lbc::prelude::Input;
use lbc::prelude::{Block, Content, Control, HeaderSize, Subtitle, Tag, TagColor, Title};
use leptos::prelude::*;
use std::sync::Arc;

/// Example page showcasing the Control form component.
#[allow(non_snake_case)]
pub fn FormControlPage() -> impl IntoView {
    #[allow(unused)]
    let (typed_text, set_typed_text) = signal(String::new());
    let (typed_text2, set_typed_text2) = signal("Read only".to_string());

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Control"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Control"</Subtitle>
                <Control>
                    <Input name="name1" value=typed_text placeholder="Your name" update=Arc::new(move |value| set_typed_text.set(value))/>
                </Control>

                <div class="mt-4"></div>

                <Subtitle size=HeaderSize::Is6>"Interactive Control (on input)"</Subtitle>
                <Control>
                    <Input readonly={true} name="name1" value=typed_text2 placeholder="Your name" update=Arc::new(move |value| set_typed_text2.set(value))/>
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
