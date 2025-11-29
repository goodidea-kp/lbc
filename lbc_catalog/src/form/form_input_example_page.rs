use lbc::prelude::{
    Block, Button, Content, Control, Field, HeaderSize, Input, InputType, Size, Subtitle, Title,
};
use leptos::prelude::{AddAnyAttr, ClassAttribute, ElementChild, Get, IntoView, Set, component, signal, view};
use std::rc::Rc;
use lbc::util::{TestAttr};

/// Example page showcasing the Input form component.
#[component]
pub fn FormInputPage() -> impl IntoView {
    let (text_value, set_text_value) = signal(String::new());
    let (number_value, set_number_value) = signal("0".to_string());

    // Signals for the "Sizes & Styles" examples
    let (small_value, set_small_value) = signal(String::new());
    let (normal_value, set_normal_value) = signal(String::new());
    let (medium_value, set_medium_value) = signal(String::new());
    let (large_value, set_large_value) = signal(String::new());
    let (rounded_value, set_rounded_value) = signal(String::new());
    let (readonly_value, set_readonly_value) = signal("Read-only value".to_string());
    let (disabled_value, set_disabled_value) = signal("Disabled value".to_string());
    let (static_value, set_static_value) = signal("Static value".to_string());

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Input"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Input"</Subtitle>
                <Field label="Your name" help="Type something here">
                    <Control>
                        <Input
                            test_attr = TestAttr::new("input","input")
                            name="name"
                            value=text_value
                            placeholder="Your name"
                            update=Rc::new(move |v| { lbc::lbc_log!("[Page] update(name) -> '{}'", v); set_text_value.set(v) })
                        />
                    </Control>
                    <Control>
                        <Button size=Size::Small on:click=move |_| { lbc::lbc_log!("[Page] Clear clicked for name"); set_text_value.set(String::new()) }>
                            "Clear"
                        </Button>
                    </Control>
                </Field>
                <p class="help">"Entered: " {move || text_value.get()}</p>

                <Subtitle size=HeaderSize::Is6>"Sizes & Styles"</Subtitle>
                <Field grouped=true multiline=true>
                    <Control>
                        <Input
                            name="small"
                            value=small_value
                            placeholder="Small"
                            size=Size::Small
                            update=Rc::new(move |v| { lbc::lbc_log!("[Page] update(small) -> '{}'", v); set_small_value.set(v) })
                        />
                        <p class="help">"Small: " {move || small_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="normal"
                            value=normal_value
                            placeholder="Normal"
                            size=Size::Normal
                            update=Rc::new(move |v| { lbc::lbc_log!("[Page] update(normal) -> '{}'", v); set_normal_value.set(v) })
                        />
                        <p class="help">"Normal: " {move || normal_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="medium"
                            value=medium_value
                            placeholder="Medium"
                            size=Size::Medium
                            update=Rc::new(move |v| { lbc::lbc_log!("[Page] update(medium) -> '{}'", v); set_medium_value.set(v) })
                        />
                        <p class="help">"Medium: " {move || medium_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="large"
                            value=large_value
                            placeholder="Large"
                            size=Size::Large
                            update=Rc::new(move |v| { lbc::lbc_log!("[Page] update(large) -> '{}'", v); set_large_value.set(v) })
                        />
                        <p class="help">"Large: " {move || large_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="rounded"
                            value=rounded_value
                            placeholder="Rounded"
                            rounded=true
                            update=Rc::new(move |v| { lbc::lbc_log!("[Page] update(rounded) -> '{}'", v); set_rounded_value.set(v) })
                        />
                        <p class="help">"Rounded: " {move || rounded_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="readonly"
                            value=readonly_value
                            readonly=true
                            update=Rc::new(move |v| { lbc::lbc_log!("[Page] update(readonly) -> '{}'", v); set_readonly_value.set(v) })
                        />
                        <p class="help">"Read-only: " {move || readonly_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="disabled"
                            value=disabled_value
                            disabled=true
                            update=Rc::new(move |v| { lbc::lbc_log!("[Page] update(disabled) -> '{}'", v); set_disabled_value.set(v) })
                        />
                        <p class="help">"Disabled: " {move || disabled_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="static"
                            value=static_value
                            r#static=true
                            update=Rc::new(move |v| { lbc::lbc_log!("[Page] update(static) -> '{}'", v); set_static_value.set(v) })
                        />
                        <p class="help">"Static: " {move || static_value.get()}</p>
                    </Control>
                </Field>

                <Subtitle size=HeaderSize::Is6>"Number Input"</Subtitle>
                <Field label="Amount" help="Up to 2 decimal places">
                    <Control>
                        <Input
                            name="amount"
                            value=number_value
                            r#type=InputType::Number
                            step=1.0
                            placeholder="123.45"
                            update=Rc::new(move |v| { lbc::lbc_log!("[Page] update(amount) -> '{}'", v); set_number_value.set(v) })
                        />
                    </Control>
                </Field>
                <p class="help">"Amount entered: " {move || number_value.get()}</p>
            </Content>
        </Block>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::{RenderHtml, GetUntracked};

    // Verifies that clicking the "Clear" button empties the Input with selector 'attr:input'
    #[test]
    fn input_clears_value_on_clear_button() {
        // initial non-empty value to verify change after "Clear"
        let (text_value, set_text_value) = signal("John".to_string());

        // Build HTML before clicking Clear (value should be non-empty)
        let html_before = view! {
            <Field label="Your name">
                <Control>
                    <Input
                        test_attr = TestAttr::new("input","input")
                        name="name"
                        value=text_value.get_untracked()
                        placeholder="Your name"
                        update=Rc::new({ let set_text_value = set_text_value.clone(); move |v| set_text_value.set(v) })
                    />
                </Control>
                <Control>
                    <Button size=Size::Small on:click=move |_| set_text_value.set(String::new())>
                        "Clear"
                    </Button>
                </Control>
            </Field>
        }.to_html();

        // Ensure the initial value is present
        assert!(html_before.contains(r#"value="John""#), "expected initial input value in: {}", html_before);

        // Emulate clicking the Clear button by invoking the setter (same effect as on:click)
        set_text_value.set(String::new());

        // Re-render after the change; now the input value should be empty
        let html_after = view! {
            <Field label="Your name">
                <Control>
                    <Input
                        test_attr = TestAttr::new("input","input")
                        name="name"
                        value=text_value.get_untracked()
                        placeholder="Your name"
                        update=Rc::new({ let set_text_value = set_text_value.clone(); move |v| set_text_value.set(v) })
                    />
                </Control>
                <Control>
                    <Button size=Size::Small on:click=move |_| set_text_value.set(String::new())>
                        "Clear"
                    </Button>
                </Control>
            </Field>
        }.to_html();

        assert!(html_after.contains(r#"value="""#), "expected empty input value after Clear in: {}", html_after);
    }
}
