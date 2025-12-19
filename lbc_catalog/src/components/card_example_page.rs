use lbc::prelude::Buttons;
use lbc::prelude::{
    Block, Card, CardContent, CardFooter, CardHeader, CardImage, Content, HeaderSize, Image, Tag,
    TagColor, Title,
};
use leptos::ev::MouseEvent;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::Set;
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, IntoView, OnAttribute, Update, component, signal, view,
};

#[component]
pub fn CardPage() -> impl IntoView {
    let (liked, set_liked) = signal(false);
    let (theme, set_theme) = signal("light".to_string());

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Card"</Title>

            <Buttons>
                <lbc::prelude::Button
                    classes="is-primary"
                    on:click=move |_| set_theme.set(
                        if theme.get() == "light" {
                            "dark".to_string()
                        } else {
                            "light".to_string()
                        }
                    )
                >
                    "Toggle Card Theme"
                </lbc::prelude::Button>
            </Buttons>

            <Card data_theme=theme>
                <CardHeader classes="p-2">
                    <p class="card-header-title">"Leptos Card"</p>
                </CardHeader>

                <CardImage>
                    <figure class="image is-4by3">
                        <Image
                            src="https://images.unsplash.com/photo-1518779578993-ec3579fee39f?w=1200&q=80&auto=format&fit=crop"
                            alt="Developer workstation with laptop and UI on screen"
                            style="width:100%;height:100%;object-fit:cover;"
                        />
                    </figure>
                </CardImage>

                <CardContent>
                    <Content>
                        <Title size=HeaderSize::Is5>"Beautiful UI with Bulma + Leptos"</Title>
                        <lbc::prelude::Subtitle size=HeaderSize::Is6 classes="has-text-grey">
                            "Build responsive, reactive UIs with ergonomics."
                        </lbc::prelude::Subtitle>

                        <p>
                            "This card demonstrates how you can compose Bulma components with Leptos signals. "
                            "Try toggling the theme or clicking Like below."
                        </p>

                        <ul>
                            <li>"Lightweight, idiomatic Rust components"</li>
                            <li>"Reactive state updates"</li>
                            <li>"Bulma styling out of the box"</li>
                        </ul>

                        <div class="tags">
                            <Tag color=TagColor::Info>"Bulma"</Tag>
                            <Tag color=TagColor::Link>"Leptos"</Tag>
                            <Tag color=TagColor::Success>"Rust"</Tag>
                        </div>
                    </Content>
                </CardContent>

                <CardFooter>
                    <a
                        class="card-footer-item"
                        href="#"
                        on:click=move |e: MouseEvent| {
                            e.prevent_default();
                            set_liked.update(|v| *v = !*v);
                        }
                    >
                        {move || if liked.get() { "Liked ✓" } else { "Like" }}
                    </a>
                    <a
                        class="card-footer-item"
                        href="#"
                        on:click=move |e: MouseEvent| {
                            e.prevent_default();
                            set_theme.update(|t| {
                                if t == "light" {
                                    *t = "dark".to_string();
                                } else {
                                    *t = "light".to_string();
                                }
                            });
                        }
                    >
                        "Theme"
                    </a>
                    <a class="card-footer-item" href="https://bulma.io" target="_blank" rel="noopener">"Docs"</a>
                </CardFooter>
            </Card>

            <p class="help mt-3">
                "Liked: " {move || if liked.get() { "yes" } else { "no" }}
            </p>
        </Block>
    }
}
