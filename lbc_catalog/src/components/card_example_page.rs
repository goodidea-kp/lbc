use std::cell::Cell;
use std::rc::Rc;

use lbc::prelude::{
    Block, Card, CardContent, CardFooter, CardHeader, CardImage, Content, HeaderSize, Image, Tag,
    TagColor, Title,
};
use leptos::html;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, GetUntracked, IntoView, NodeRef, NodeRefAttribute,
    Set, component, signal, view,
};

#[component]
pub fn CardPage() -> impl IntoView {
    let (liked, set_liked) = signal(false);
    let (theme, set_theme) = signal(String::new());

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually on wasm32.
    let toggle_theme_button_ref: NodeRef<html::Button> = NodeRef::new();
    let like_link_ref: NodeRef<html::A> = NodeRef::new();
    let theme_link_ref: NodeRef<html::A> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::Event;

        fn attach_button_click_once(
            button_ref: NodeRef<html::Button>,
            has_attached: Rc<Cell<bool>>,
            on_click: Rc<dyn Fn()>,
        ) {
            Effect::new(move |_| {
                if has_attached.get() {
                    return;
                }

                let Some(button_element) = button_ref.get() else {
                    return;
                };

                let on_click_for_event = on_click.clone();
                let click_closure: Closure<dyn FnMut(Event)> =
                    Closure::wrap(Box::new(move |event: Event| {
                        event.prevent_default();
                        (on_click_for_event)();
                    }));

                button_element
                    .add_event_listener_with_callback(
                        "click",
                        click_closure.as_ref().unchecked_ref(),
                    )
                    .ok();

                has_attached.set(true);
                click_closure.forget();
            });
        }

        fn attach_anchor_click_once(
            anchor_ref: NodeRef<html::A>,
            has_attached: Rc<Cell<bool>>,
            on_click: Rc<dyn Fn()>,
        ) {
            Effect::new(move |_| {
                if has_attached.get() {
                    return;
                }

                let Some(anchor_element) = anchor_ref.get() else {
                    return;
                };

                let on_click_for_event = on_click.clone();
                let click_closure: Closure<dyn FnMut(Event)> =
                    Closure::wrap(Box::new(move |event: Event| {
                        event.prevent_default();
                        (on_click_for_event)();
                    }));

                anchor_element
                    .add_event_listener_with_callback(
                        "click",
                        click_closure.as_ref().unchecked_ref(),
                    )
                    .ok();

                has_attached.set(true);
                click_closure.forget();
            });
        }

        let toggle_theme_attached = Rc::new(Cell::new(false));
        let like_attached = Rc::new(Cell::new(false));
        let theme_attached = Rc::new(Cell::new(false));

        attach_button_click_once(
            toggle_theme_button_ref.clone(),
            toggle_theme_attached,
            Rc::new({
                let theme = theme.clone();
                let set_theme = set_theme.clone();
                move || {
                    if theme.get_untracked().trim().is_empty() {
                        set_theme.set("has-background-primary-light".to_string());
                    } else {
                        set_theme.set(String::new());
                    }
                }
            }),
        );

        attach_anchor_click_once(
            like_link_ref.clone(),
            like_attached,
            Rc::new({
                let liked = liked.clone();
                let set_liked = set_liked.clone();
                move || set_liked.set(!liked.get_untracked())
            }),
        );

        attach_anchor_click_once(
            theme_link_ref.clone(),
            theme_attached,
            Rc::new({
                let theme = theme.clone();
                let set_theme = set_theme.clone();
                move || {
                    if theme.get_untracked().trim().is_empty() {
                        set_theme.set("has-background-primary-light".to_string());
                    } else {
                        set_theme.set(String::new());
                    }
                }
            }),
        );
    }

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Card"</Title>

            <div class="buttons">
                <button
                    node_ref=toggle_theme_button_ref
                    class="button"
                    type="button"
                >
                    "Toggle Theme"
                </button>
            </div>

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
                        node_ref=like_link_ref
                        class="card-footer-item"
                        href="#"
                    >
                        {move || if liked.get() { "Liked ✓" } else { "Like" }}
                    </a>
                    <a
                        node_ref=theme_link_ref
                        class="card-footer-item"
                        href="#"
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
