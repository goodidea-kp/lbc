use lbc::prelude::*;
use lbc::util::TestAttr;
use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::prelude::{ClassAttribute, ElementChild, Get, IntoView, Set, component, signal, view};

#[cfg(target_arch = "wasm32")]
fn console_log(message: &str) {
    use leptos::wasm_bindgen::JsValue;
    use leptos::web_sys::console;

    console::log_1(&JsValue::from_str(message));
}

#[cfg(not(target_arch = "wasm32"))]
fn console_log(message: &str) {
    println!("{message}");
}

/// Example page showcasing the Input form component.
#[component]
pub fn FormInputPage() -> impl IntoView {
    console_log("[FormInputPage] render start");

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
                            test_attr=TestAttr::new("input","input")
                            name="name"
                            value=text_value
                            placeholder="Your name"
                            update=Callback::new(move |value| {
                                console_log(&format!("[FormInputPage] update(name) -> '{value}'"));
                                lbc::lbc_log!("[Page] update(name) -> '{}'", value);
                                set_text_value.set(value);
                            })
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
                            update=Callback::new(move |value| {
                                console_log(&format!("[FormInputPage] update(small) -> '{value}'"));
                                lbc::lbc_log!("[Page] update(small) -> '{}'", value);
                                set_small_value.set(value);
                            })
                        />
                        <p class="help">"Small: " {move || small_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="normal"
                            value=normal_value
                            placeholder="Normal"
                            size=Size::Normal
                            update=Callback::new(move |value| {
                                console_log(&format!("[FormInputPage] update(normal) -> '{value}'"));
                                lbc::lbc_log!("[Page] update(normal) -> '{}'", value);
                                set_normal_value.set(value);
                            })
                        />
                        <p class="help">"Normal: " {move || normal_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="medium"
                            value=medium_value
                            placeholder="Medium"
                            size=Size::Medium
                            update=Callback::new(move |value| {
                                console_log(&format!("[FormInputPage] update(medium) -> '{value}'"));
                                lbc::lbc_log!("[Page] update(medium) -> '{}'", value);
                                set_medium_value.set(value);
                            })
                        />
                        <p class="help">"Medium: " {move || medium_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="large"
                            value=large_value
                            placeholder="Large"
                            size=Size::Large
                            update=Callback::new(move |value| {
                                console_log(&format!("[FormInputPage] update(large) -> '{value}'"));
                                lbc::lbc_log!("[Page] update(large) -> '{}'", value);
                                set_large_value.set(value);
                            })
                        />
                        <p class="help">"Large: " {move || large_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="rounded"
                            value=rounded_value
                            placeholder="Rounded"
                            rounded=true
                            update=Callback::new(move |value| {
                                console_log(&format!("[FormInputPage] update(rounded) -> '{value}'"));
                                lbc::lbc_log!("[Page] update(rounded) -> '{}'", value);
                                set_rounded_value.set(value);
                            })
                        />
                        <p class="help">"Rounded: " {move || rounded_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="readonly"
                            value=readonly_value
                            readonly=true
                            update=Callback::new(move |value| {
                                console_log(&format!("[FormInputPage] update(readonly) -> '{value}'"));
                                lbc::lbc_log!("[Page] update(readonly) -> '{}'", value);
                                set_readonly_value.set(value);
                            })
                        />
                        <p class="help">"Read-only: " {move || readonly_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="disabled"
                            value=disabled_value
                            disabled=true
                            update=Callback::new(move |value| {
                                console_log(&format!("[FormInputPage] update(disabled) -> '{value}'"));
                                lbc::lbc_log!("[Page] update(disabled) -> '{}'", value);
                                set_disabled_value.set(value);
                            })
                        />
                        <p class="help">"Disabled: " {move || disabled_value.get()}</p>
                    </Control>
                    <Control>
                        <Input
                            name="static"
                            value=static_value
                            r#static=true
                            update=Callback::new(move |value| {
                                console_log(&format!("[FormInputPage] update(static) -> '{value}'"));
                                lbc::lbc_log!("[Page] update(static) -> '{}'", value);
                                set_static_value.set(value);
                            })
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
                            update=Callback::new(move |value| {
                                console_log(&format!("[FormInputPage] update(amount) -> '{value}'"));
                                lbc::lbc_log!("[Page] update(amount) -> '{}'", value);
                                set_number_value.set(value);
                            })
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
    use leptos::prelude::{GetUntracked, RenderHtml};

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
                        test_attr=TestAttr::new("input","input")
                        name="name"
                        value=text_value.get_untracked()
                        placeholder="Your name"
                        update=Callback::new({
                            let set_text_value = set_text_value.clone();
                            move |value| set_text_value.set(value)
                        })
                    />
                </Control>
                <Control>
                    <button class="button is-small" type="button">"Clear"</button>
                </Control>
            </Field>
        }
        .to_html();

        // Ensure the initial value is present
        assert!(
            html_before.contains(r#"value="John""#),
            "expected initial input value in: {}",
            html_before
        );

        // Emulate clicking the Clear button by invoking the setter (same effect as click)
        set_text_value.set(String::new());

        // Re-render after the change; now the input value should be empty
        let html_after = view! {
            <Field label="Your name">
                <Control>
                    <Input
                        test_attr=TestAttr::new("input","input")
                        name="name"
                        value=text_value.get_untracked()
                        placeholder="Your name"
                        update=Callback::new({
                            let set_text_value = set_text_value.clone();
                            move |value| set_text_value.set(value)
                        })
                    />
                </Control>
                <Control>
                    <button class="button is-small" type="button">"Clear"</button>
                </Control>
            </Field>
        }
        .to_html();

        assert!(
            html_after.contains(r#"value="""#),
            "expected empty input value after Clear in: {}",
            html_after
        );
    }
}
