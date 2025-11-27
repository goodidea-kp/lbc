use std::sync::Arc;

use lbc::prelude::{Block, Checkbox, Content, HeaderSize, Subtitle, Title};
use leptos::prelude::*;

/// Example page showcasing the Checkbox form component.
pub fn FormCheckboxPage() -> impl IntoView {
    let (is_checked, set_is_checked) = signal(false);

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Checkbox"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Checkbox"</Subtitle>
                <Checkbox
                    name="terms"
                    checked=is_checked
                    update=Arc::new(move |next| set_is_checked.set(next))
                    classes="mr-2"
                >
                    {"I agree to the terms and conditions"}
                </Checkbox>

                <p class="mt-3">
                    <strong>"Checked: "</strong>
                    { move || if is_checked.get() { "true" } else { "false" } }
                </p>

                <div class="mt-4"></div>

                <Subtitle size=HeaderSize::Is6>"Disabled"</Subtitle>
                <Checkbox name="disabled_demo" checked=true disabled=true>
                    {"Cannot change this option"}
                </Checkbox>
            </Content>
        </Block>
    }
}
