use std::cell::Cell;
use std::rc::Rc;

use lbc::prelude::{
    Block, Content, HeaderSize, Icon, Panel, PanelBlock, PanelTabs, Subtitle, Title,
};
use leptos::html;
use leptos::prelude::{
    AriaAttributes, ClassAttribute, Effect, ElementChild, Get, IntoView, NodeRef, NodeRefAttribute,
    Set, Signal, component, signal, view,
};

#[component]
pub fn PanelPage() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(0usize);
    let (active_block, set_active_block) = signal(0usize);

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually on wasm32.
    let all_tab_ref: NodeRef<html::A> = NodeRef::new();
    let public_tab_ref: NodeRef<html::A> = NodeRef::new();
    let private_tab_ref: NodeRef<html::A> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys::Event;

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
                    .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
                    .ok();

                has_attached.set(true);
                click_closure.forget();
            });
        }

        attach_anchor_click_once(
            all_tab_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_tab = set_active_tab.clone();
                move || set_active_tab.set(0)
            }),
        );

        attach_anchor_click_once(
            public_tab_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_tab = set_active_tab.clone();
                move || set_active_tab.set(1)
            }),
        );

        attach_anchor_click_once(
            private_tab_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_tab = set_active_tab.clone();
                move || set_active_tab.set(2)
            }),
        );
    }

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Panel"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Panel with Heading"</Subtitle>
                <Panel heading="Repositories">
                    <PanelTabs>
                        <a
                            node_ref=all_tab_ref
                            href="#"
                            class=move || if active_tab.get() == 0 { "is-active" } else { "" }
                        >
                            "All"
                        </a>
                        <a
                            node_ref=public_tab_ref
                            href="#"
                            class=move || if active_tab.get() == 1 { "is-active" } else { "" }
                        >
                            "Public"
                        </a>
                        <a
                            node_ref=private_tab_ref
                            href="#"
                            class=move || if active_tab.get() == 2 { "is-active" } else { "" }
                        >
                            "Private"
                        </a>
                    </PanelTabs>

                    <PanelBlock
                        active=Signal::derive(move || active_block.get() == 0)
                        on_click=Rc::new(move || set_active_block.set(0))
                    >
                        <Icon classes="panel-icon"><i class="fas fa-book" aria-hidden="true"></i></Icon>
                        "bulma"
                    </PanelBlock>
                    <PanelBlock
                        active=Signal::derive(move || active_block.get() == 1)
                        on_click=Rc::new(move || set_active_block.set(1))
                    >
                        <Icon classes="panel-icon"><i class="fas fa-book" aria-hidden="true"></i></Icon>
                        "leptos"
                    </PanelBlock>
                    <PanelBlock
                        tag="a"
                        active=Signal::derive(move || active_block.get() == 2)
                        on_click=Rc::new(move || set_active_block.set(2))
                    >
                        <Icon classes="panel-icon"><i class="fas fa-book" aria-hidden="true"></i></Icon>
                        "lbc"
                    </PanelBlock>
                </Panel>
            </Content>
        </Block>
    }
}
