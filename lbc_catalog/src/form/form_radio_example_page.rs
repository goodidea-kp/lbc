use lbc::prelude::{Block, Content, Control, Field, HeaderSize, Radio, Subtitle, Title};
use leptos::prelude::{ClassAttribute, ElementChild, Get, IntoView, Set, component, view, signal};
use std::sync::Arc;

/// Example page showcasing the Radio form component.
#[component]
pub fn FormRadioPage() -> impl IntoView {
    let (selected, set_selected) = signal(String::from("b"));

    let update = Arc::new(move |v: String| set_selected.set(v));
    // Clone update for each radio to avoid moving it into multiple closures
    let update_a = update.clone();
    let update_b = update.clone();
    let update_c = update.clone();

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Radio"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Radios"</Subtitle>

                <Field>
                    <Control>
                        <Radio name="group1" value="a" checked_value=selected.get() update=update_a>
                            "Choice A"
                        </Radio>
                    </Control>
                    <Control>
                        <Radio name="group1" value="b" checked_value=selected.get() update=update_b>
                            "Choice B"
                        </Radio>
                    </Control>
                    <Control>
                        <Radio name="group1" value="c" checked_value=selected.get() update=update_c>
                            "Choice C"
                        </Radio>
                    </Control>
                </Field>

                <p class="help">"Selected: " {move || selected.get()}</p>

                <Subtitle size=HeaderSize::Is6>"Disabled"</Subtitle>
                <Field>
                    <Control>
                        <Radio name="group2" value="x" checked_value="x".to_string() disabled=true update=Arc::new(|_| {})>
                            "Disabled X"
                        </Radio>
                    </Control>
                </Field>
            </Content>
        </Block>
    }
}
