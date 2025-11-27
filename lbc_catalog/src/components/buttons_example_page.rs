use lbc::prelude::{
    Alignment, Block, Button, ButtonColor, Buttons, HeaderSize, Size, Subtitle, Title,
};
use leptos::prelude::{ClassAttribute, IntoView, component, view};

#[component]
pub fn ButtonsPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Buttons"</Title>

            <Subtitle size=HeaderSize::Is6>"Basic Group"</Subtitle>
            <Buttons>
                <Button classes="is-primary">"Primary"</Button>
                <Button classes="is-link">"Link"</Button>
                <Button classes="is-info">"Info"</Button>
                <Button classes="is-success">"Success"</Button>
                <Button classes="is-warning">"Warning"</Button>
                <Button classes="is-danger">"Danger"</Button>
            </Buttons>

            <div class="mt-3"></div>

            <Subtitle size=HeaderSize::Is6>"Small Size Group"</Subtitle>
            <Buttons size=Size::Small>
                <Button>"One"</Button>
                <Button>"Two"</Button>
                <Button>"Three"</Button>
            </Buttons>

            <div class="mt-3"></div>

            <Subtitle size=HeaderSize::Is6>"With Addons"</Subtitle>
            <Buttons addons=true>
                <Button classes="is-success">"Yes"</Button>
                <Button classes="is-danger">"No"</Button>
            </Buttons>

            <div class="mt-3"></div>

            <Subtitle size=HeaderSize::Is6>"Alignment: Centered"</Subtitle>
            <Buttons alignment=Alignment::Centered>
                <Button color=ButtonColor::Primary>"Centered"</Button>
                <Button>"Neutral"</Button>
                <Button color=ButtonColor::Link>"Action"</Button>
            </Buttons>

            <div class="mt-3"></div>

            <Subtitle size=HeaderSize::Is6>"Alignment: Right"</Subtitle>
            <Buttons alignment=Alignment::Right>
                <Button classes="is-light">"Cancel"</Button>
                <Button classes="is-info">"Save"</Button>
            </Buttons>
        </Block>
    }
}
