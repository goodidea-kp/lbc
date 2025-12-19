use leptos::callback::Callback;
use lbc::prelude::{
    Block, Content, Control, Field, HeaderSize, MultiSelect, Select, Subtitle, Title,
};
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, GetUntracked, IntoView, Set, component, signal, view,
};

/// Example page showcasing the Select and MultiSelect form components.
#[component]
pub fn FormSelectPage() -> impl IntoView {
    // Single-select value
    let (selected, set_selected) = signal(String::from("b"));
    let update_select = Callback::new(move |v: String| set_selected.set(v));

    // Multi-select values
    let (selected_list, set_selected_list) = signal(vec!["a".to_string(), "c".to_string()]);
    let update_multi = Callback::new(move |v: Vec<String>| set_selected_list.set(v));

    // Snapshot values for passing into props (avoid reading signals in non-tracking context).
    let selected_value = selected.get_untracked();
    let selected_list_value = selected_list.get_untracked();

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: Select"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Select"</Subtitle>
                <Field label="Favorite letter" help="Choose one option">
                    <Control>
                        <Select name="letters" value=selected_value update=update_select.clone()>
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
                        <MultiSelect
                            name="letters-multi"
                            value=selected_list_value
                            list_size=6
                            update=update_multi.clone()
                        >
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
