use lbc::prelude::{Block, Content, Dropdown, HeaderSize, Title};
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, IntoAny, IntoView, OnAttribute, Set, component, signal, view,
};

#[component]
pub fn DropdownPage() -> impl IntoView {
    let (selected, set_selected) = signal(String::from("-"));

    let select_item = move |label: &'static str| {
        let set_selected = set_selected.clone();
        move |_| set_selected.set(label.to_string())
    };

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Dropdown"</Title>

            <Content>
                <p class="subtitle is-6">"Clickable Dropdown"</p>
                <Dropdown
                    classes=""
                    button_classes=""
                    button=Box::new(|| view! { "Open menu" }.into_any())
                >
                    <a class="dropdown-item" on:click=select_item("Overview")>"Overview"</a>
                    <a class="dropdown-item" on:click=select_item("Modifiers")>"Modifiers"</a>
                    <a class="dropdown-item" on:click=select_item("Grid")>"Grid"</a>
                    <hr class="dropdown-divider"/>
                    <a class="dropdown-item" on:click=select_item("Elements")>"Elements"</a>
                    <a class="dropdown-item" on:click=select_item("Components")>"Components"</a>
                </Dropdown>

                <div class="mt-3"></div>

                <p class="subtitle is-6">"Hoverable Dropdown"</p>
                <Dropdown
                    hoverable=true
                    button_classes="is-link"
                    button=Box::new(|| view! {
                        <span>"Hover me"</span>
                    }.into_any())
                >
                    <a class="dropdown-item" on:click=select_item("First")>"First"</a>
                    <a class="dropdown-item" on:click=select_item("Second")>"Second"</a>
                    <a class="dropdown-item" on:click=select_item("Third")>"Third"</a>
                </Dropdown>

                <p class="help mt-3">"Selected: " {move || selected.get()}</p>
            </Content>
        </Block>
    }
}
