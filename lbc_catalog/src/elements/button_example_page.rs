/*!
Example page: Button

AI Pair Programming Notes:
- Single-responsibility component focused on demonstrating the Button API.
- Keep imports minimal and explicit to reduce cognitive load.
- Keep examples deterministic and small; avoid hidden state outside this module.
*/

use lbc::prelude::{Block, Title, HeaderSize, Buttons, Button, ButtonColor, Size};
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, IntoView, Update, component, create_signal, view,
};

#[component]
pub fn ButtonPage() -> impl IntoView {
    let (is_loading, set_is_loading) = create_signal(false);
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Button"</Title>
            <Buttons>
                <Button color=ButtonColor::Primary>"Primary"</Button>
                <Button color=ButtonColor::Success size=Size::Small>"Small Success"</Button>
                <Button
                    loading=is_loading
                    on_click=std::rc::Rc::new(move |_| set_is_loading.update(|value| *value = !*value))
                >
                    {move || if is_loading.get() { "Stop Loading" } else { "Start Loading" }}
                </Button>
                <Button outlined=true color=ButtonColor::Danger>"Outlined Danger"</Button>
            </Buttons>
        </Block>
    }
}
