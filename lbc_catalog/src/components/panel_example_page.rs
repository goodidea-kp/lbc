use leptos::prelude::{component, view, IntoView, ClassAttribute, ElementChild, Get, Set, OnAttribute, Signal, AriaAttributes, create_signal};
use lbc::prelude::{Block, Content, HeaderSize, Subtitle, Title, Panel, PanelBlock, PanelTabs, Icon};
use std::rc::Rc;

#[component]
pub fn PanelPage() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal(0usize);
    let (active_block, set_active_block) = create_signal(0usize);
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Panel"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Panel with Heading"</Subtitle>
                <Panel heading="Repositories">
                    <PanelTabs>
                        <a
                            class=move || if active_tab.get() == 0 { "is-active" } else { "" }
                            on:click=move |_| set_active_tab.set(0)
                        >"All"</a>
                        <a
                            class=move || if active_tab.get() == 1 { "is-active" } else { "" }
                            on:click=move |_| set_active_tab.set(1)
                        >"Public"</a>
                        <a
                            class=move || if active_tab.get() == 2 { "is-active" } else { "" }
                            on:click=move |_| set_active_tab.set(2)
                        >"Private"</a>
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
