use lbc::prelude::{Block, Content, Dropdown, HeaderSize, Title};
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, IntoAny, IntoView, component, signal, view,
};

#[component]
pub fn DropdownPage() -> impl IntoView {
    let (selected, set_selected) = signal(String::from("-"));

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
                    <a class="dropdown-item" href="#">"Overview"</a>
                    <a class="dropdown-item" href="#">"Modifiers"</a>
                    <a class="dropdown-item" href="#">"Grid"</a>
                    <hr class="dropdown-divider"/>
                    <a class="dropdown-item" href="#">"Elements"</a>
                    <a class="dropdown-item" href="#">"Components"</a>
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
                    <a class="dropdown-item" href="#">"First"</a>
                    <a class="dropdown-item" href="#">"Second"</a>
                    <a class="dropdown-item" href="#">"Third"</a>
                </Dropdown>

                <p class="help mt-3">"Selected: " {move || selected.get()}</p>
            </Content>
        </Block>
    }
}
