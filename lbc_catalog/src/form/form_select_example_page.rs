use leptos::prelude::{component, view, IntoView, ClassAttribute, ElementChild, Get, Set, create_signal};
use lbc::prelude::{Block, Content, Control, Field, HeaderSize, MultiSelect, Select, Subtitle, Title};
use std::sync::Arc;

/// Example page showcasing the Select and MultiSelect form components.
#[component]
pub fn FormSelectPage() -> impl IntoView {
    // Single-select value
    let (selected, set_selected) = create_signal(String::from("b"));
    let update_select = Arc::new(move |v: String| set_selected.set(v));

    // Multi-select values
    let (selected_list, set_selected_list) = create_signal(vec!["a".to_string(), "c".to_string()]);
    let update_multi = Arc::new(move |v: Vec<String>| set_selected_list.set(v));

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Select"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Select"</Subtitle>
                <Field label="Favorite letter" help="Choose one option">
                    <Control>
                        <Select name="letters" value=selected.get() update=update_select.clone()>
                            <option value="a">"A"</option>
                            <option value="b">"B"</option>
                            <option value="c">"C"</option>
                        </Select>
                    </Control>
                </Field>
                <p class="help">"Selected: " {move || selected.get()}</p>

                <Subtitle size=HeaderSize::Is6>"Multi Select"</Subtitle>
                <Field label="Pick several" help="You can select multiple options">
                    <Control>
                        <MultiSelect name="letters-multi" value=selected_list.get() list_size=6 update=update_multi.clone()>
                            <option value="a">"Alpha"</option>
                            <option value="b">"Bravo"</option>
                            <option value="c">"Charlie"</option>
                            <option value="d">"Delta"</option>
                            <option value="e">"Echo"</option>
                            <option value="f">"Foxtrot"</option>
                        </MultiSelect>
                    </Control>
                </Field>
                <p class="help">
                    "Selected: "
                    {move || selected_list.get().join(", ")}
                </p>
            </Content>
        </Block>
    }
}
