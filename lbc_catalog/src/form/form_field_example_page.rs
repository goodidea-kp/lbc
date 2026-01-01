use lbc::prelude::{Block, Content, Control, Field, HeaderSize, Subtitle, Title};
use lbc::prelude::{Input, InputType};
use leptos::callback::Callback;
use leptos::prelude::*;

/// Example page showcasing the Field form container component.
#[allow(non_snake_case)]
pub fn FormFieldPage() -> impl IntoView {
    let (text_value, set_text_value) = signal(String::new());
    let (email_value, set_email_value) = signal(String::new());
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Field"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Field with label and help"</Subtitle>
                <Field label="Name" help="Please enter your full name">
                    <Control>
                        <Input
                            name="name"
                            value=text_value
                            placeholder="Your name"
                            update=Callback::new(move |value| {
                                set_text_value.set(value);
                            })
                        />
                    </Control>
                </Field>

                <div class="mt-4"></div>

                <Subtitle size=HeaderSize::Is6>"Horizontal Field"</Subtitle>
                <Field label="Email" horizontal=true help="We'll never share your email">
                    <Control>
                        <Input
                            r#type=InputType::Email
                            name="email"
                            value=email_value
                            placeholder="you@example.com"
                            update=Callback::new(move |value| {
                                set_email_value.set(value);
                            })
                        />
                    </Control>
                </Field>
            </Content>
        </Block>
    }
}
