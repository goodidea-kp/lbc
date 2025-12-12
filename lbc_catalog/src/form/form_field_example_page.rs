use lbc::prelude::{Block, Content, Control, Field, HeaderSize, Subtitle, Title};
use leptos::prelude::*;

/// Example page showcasing the Field form container component.
#[allow(non_snake_case)]
pub fn FormFieldPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Field"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Field with label and help"</Subtitle>
                <Field label="Name" help="Please enter your full name">
                    <Control>
                        <input class="input" type="text" placeholder="Your name"/>
                    </Control>
                </Field>

                <div class="mt-4"></div>

                <Subtitle size=HeaderSize::Is6>"Horizontal Field"</Subtitle>
                <Field label="Email" horizontal=true help="We'll never share your email">
                    <Control>
                        <input class="input" type="email" placeholder="you@example.com"/>
                    </Control>
                </Field>
            </Content>
        </Block>
    }
}
