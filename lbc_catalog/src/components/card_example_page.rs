use lbc::prelude::{
    Block, Button, Card, CardContent, CardFooter, CardHeader, CardImage, Content,
    HeaderSize, Image, Subtitle, Tag, TagColor, Title,
};
use leptos::prelude::{
    AddAnyAttr, ClassAttribute, ElementChild, Get, GetUntracked, IntoView, OnAttribute,
    Set, component, view, signal
};

#[component]
pub fn CardPage() -> impl IntoView {
    let (liked, set_liked) = signal(false);
    let (theme, set_theme) = signal(String::new());

    let toggle_like = {
        let set_liked = set_liked.clone();
        move |_| set_liked.set(!liked.get_untracked())
    };

    let cycle_theme = move |_| {
        // Cycle between no theme and is-primary
        if theme.get_untracked().trim().is_empty() {
            set_theme.set("has-background-primary-light".to_string());
        } else {
            set_theme.set(String::new());
        }
    };

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Card"</Title>

            <lbc::prelude::Buttons>
                <Button on:click=cycle_theme>"Toggle Theme"</Button>
            </lbc::prelude::Buttons>

            <Card classes=theme>
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
                        <Subtitle size=HeaderSize::Is6 classes="has-text-grey">"Build responsive, reactive UIs with ergonomics."</Subtitle>

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
                    <a class="card-footer-item" on:click=toggle_like>
                        {move || if liked.get() { "Liked ✓" } else { "Like" }}
                    </a>
                    <a class="card-footer-item" on:click=cycle_theme>"Theme"</a>
                    <a class="card-footer-item" href="https://bulma.io" target="_blank" rel="noopener">"Docs"</a>
                </CardFooter>
            </Card>

            <p class="help mt-3">
                "Liked: " {move || if liked.get() { "yes" } else { "no" }}
            </p>
        </Block>
    }
}
