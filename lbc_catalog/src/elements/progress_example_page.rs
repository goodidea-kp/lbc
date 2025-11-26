/*!
Example page: Progress

AI Pair Programming Notes:
- Single-responsibility component focused on demonstrating the Progress API.
- Keep imports minimal and explicit to reduce cognitive load.
- Keep examples deterministic and small; avoid hidden state outside this module.
*/

use lbc::prelude::{
    Block, Button, ButtonColor, Buttons, HeaderSize, Progress, Size, Subtitle, Title,
};
use leptos::prelude::{ClassAttribute, ElementChild, Get, IntoView, Set, Update, component, view, signal};
use std::rc::Rc;

#[component]
pub fn ProgressPage() -> impl IntoView {
    let (progress_value, set_progress_value) = signal(25.0);

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Progress"</Title>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Basic Progress"</Subtitle>
                <Progress max=100.0 value=15.0 />
                <Progress max=100.0 value=30.0 classes="is-primary" />
                <Progress max=100.0 value=45.0 classes="is-link" />
                <Progress max=100.0 value=60.0 classes="is-info" />
                <Progress max=100.0 value=75.0 classes="is-success" />
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Sizes"</Subtitle>
                <Progress max=100.0 value=25.0 classes="is-small is-primary" />
                <Progress max=100.0 value=50.0 classes="is-medium is-info" />
                <Progress max=100.0 value=75.0 classes="is-large is-success" />
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Indeterminate Progress"</Subtitle>
                <Progress max=100.0 value=-1.0 classes="is-primary" />
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Interactive Progress"</Subtitle>
                <Progress max=100.0 value=progress_value />
                <Buttons classes="mt-4" size=Size::Small>
                    <Button
                        color=ButtonColor::Success
                        on_click=Rc::new(move |_| set_progress_value.update(|v| *v = (*v + 10.0).min(100.0)))
                    >
                        "+10%"
                    </Button>
                    <Button
                        color=ButtonColor::Danger
                        on_click=Rc::new(move |_| set_progress_value.update(|v| *v = (*v - 10.0).max(0.0)))
                    >
                        "-10%"
                    </Button>
                    <Button
                        classes="is-light"
                        on_click=Rc::new(move |_| set_progress_value.set(0.0))
                    >
                        "Reset"
                    </Button>
                </Buttons>
                <p class="help">"Current value: " {move || format!("{:.0}%", progress_value.get())}</p>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Different Colors"</Subtitle>
                <Progress max=100.0 value=20.0 classes="is-warning" />
                <Progress max=100.0 value=40.0 classes="is-danger" />
                <Progress max=100.0 value=60.0 classes="is-dark" />
            </Block>
        </Block>
    }
}
