use leptos::prelude::{component, view, IntoView, ClassAttribute, ElementChild, Get, Set, create_signal};
use lbc::prelude::{Block, Content, Control, Field, HeaderSize, Input, InputType, Size, Subtitle, Title};
use std::rc::Rc;

/// Example page showcasing the Input form component.
#[component]
pub fn FormInputPage() -> impl IntoView {
    let (text_value, set_text_value) = create_signal(String::new());
    let (number_value, set_number_value) = create_signal("0".to_string());

    // Signals for the "Sizes & Styles" examples
    let (small_value, set_small_value) = create_signal(String::new());
    let (normal_value, set_normal_value) = create_signal(String::new());
    let (medium_value, set_medium_value) = create_signal(String::new());
    let (large_value, set_large_value) = create_signal(String::new());
    let (rounded_value, set_rounded_value) = create_signal(String::new());
    let (readonly_value, set_readonly_value) = create_signal("Read-only value".to_string());
    let (disabled_value, set_disabled_value) = create_signal("Disabled value".to_string());
    let (static_value, set_static_value) = create_signal("Static value".to_string());

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Input"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Input"</Subtitle>
                <Field label="Your name" help="Type something here">
                    <Control>
                        <Input
                            name="name"
                            value=text_value.get()
                            placeholder="Your name"
                            update=Rc::new(move |v| set_text_value.set(v))
                        />
                    </Control>
                </Field>
                <p class="help">"Entered: " {move || text_value.get()}</p>

                <Subtitle size=HeaderSize::Is6>"Sizes & Styles"</Subtitle>
                <Field grouped=true multiline=true>
                    <Control>
                        <Input
                            name="small"
                            value=small_value.get()
                            placeholder="Small"
                            size=Size::Small
                            update=Rc::new(move |v| set_small_value.set(v))
                        />
                        <p class="help">"Small: " {move || small_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="normal"
                            value=normal_value.get()
                            placeholder="Normal"
                            size=Size::Normal
                            update=Rc::new(move |v| set_normal_value.set(v))
                        />
                        <p class="help">"Normal: " {move || normal_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="medium"
                            value=medium_value.get()
                            placeholder="Medium"
                            size=Size::Medium
                            update=Rc::new(move |v| set_medium_value.set(v))
                        />
                        <p class="help">"Medium: " {move || medium_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="large"
                            value=large_value.get()
                            placeholder="Large"
                            size=Size::Large
                            update=Rc::new(move |v| set_large_value.set(v))
                        />
                        <p class="help">"Large: " {move || large_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="rounded"
                            value=rounded_value.get()
                            placeholder="Rounded"
                            rounded=true
                            update=Rc::new(move |v| set_rounded_value.set(v))
                        />
                        <p class="help">"Rounded: " {move || rounded_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="readonly"
                            value=readonly_value.get()
                            readonly=true
                            update=Rc::new(move |v| set_readonly_value.set(v))
                        />
                        <p class="help">"Read-only: " {move || readonly_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="disabled"
                            value=disabled_value.get()
                            disabled=true
                            update=Rc::new(move |v| set_disabled_value.set(v))
                        />
                        <p class="help">"Disabled: " {move || disabled_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="static"
                            value=static_value.get()
                            r#static=true
                            update=Rc::new(move |v| set_static_value.set(v))
                        />
                        <p class="help">"Static: " {move || static_value.get()}</p>
                    </Control>
                </Field>

                <Subtitle size=HeaderSize::Is6>"Number Input"</Subtitle>
                <Field label="Amount" help="Up to 2 decimal places">
                    <Control>
                        <Input
                            name="amount"
                            value=number_value.get()
                            r#type=InputType::Number
                            step=1.0
                            placeholder="123.45"
                            update=Rc::new(move |v| set_number_value.set(v))
                        />
                    </Control>
                </Field>
                <p class="help">"Amount entered: " {move || number_value.get()}</p>
            </Content>
        </Block>
    }
}
