use lbc::prelude::{Block, Content, Control, Field, HeaderSize, Radio, Subtitle, Title};
use leptos::callback::Callback;
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, GetUntracked, IntoView, Set, component, signal, view,
};

/// Example page showcasing the Radio form component.
#[component]
#[allow(non_snake_case)]
pub fn FormRadioPage() -> impl IntoView {
    let (selected, set_selected) = signal(String::from("b"));

    let update = Callback::new(move |v: String| set_selected.set(v));
    // Clone update for each radio to avoid moving it into multiple closures
    let update_a = update.clone();
    let update_b = update.clone();
    let update_c = update.clone();

    // Snapshot the current selected value without tracking to avoid reactive_graph warnings.
    // Use separate owned Strings to avoid `view!` moving a shared handle into closures.
    let checked_value_a = selected.get_untracked();
    let checked_value_b = selected.get_untracked();
    let checked_value_c = selected.get_untracked();

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Radio"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Radios"</Subtitle>

                <Field>
                    <Control>
                        <Radio
                            name="group1"
                            value="a"
                            checked_value=checked_value_a
                            update=update_a
                        >
                            "Choice A"
                        </Radio>
                    </Control>
                    <Control>
                        <Radio
                            name="group1"
                            value="b"
                            checked_value=checked_value_b
                            update=update_b
                        >
                            "Choice B"
                        </Radio>
                    </Control>
                    <Control>
                        <Radio
                            name="group1"
                            value="c"
                            checked_value=checked_value_c
                            update=update_c
                        >
                            "Choice C"
                        </Radio>
                    </Control>
                </Field>

                <p class="help">"Selected: " {move || selected.get()}</p>

                <Subtitle size=HeaderSize::Is6>"Disabled"</Subtitle>
                <Field>
                    <Control>
                        <Radio
                            name="group2"
                            value="x"
                            checked_value="x".to_string()
                            disabled=true
                            update=Callback::new(|_| {})
                        >
                            "Disabled X"
                        </Radio>
                    </Control>
                </Field>
            </Content>
        </Block>
    }
}
