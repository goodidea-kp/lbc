use std::rc::Rc;

use lbc::prelude::{
    Block, Content, HeaderSize, Icon, Panel, PanelBlock, PanelTabs, Subtitle, Title,
};
use leptos::prelude::{
    AriaAttributes, ClassAttribute, ElementChild, Get, IntoView, Set, Signal, component, signal,
    view,
};

#[component]
pub fn PanelPage() -> impl IntoView {
    #[allow(unused)]
    let (active_tab, set_active_tab) = signal(0usize);
    let (active_block, set_active_block) = signal(0usize);

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Panel"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Panel with Heading"</Subtitle>
                <Panel heading="Repositories">
                    <PanelTabs>
                        <a
                            href="#"
                            class=move || if active_tab.get() == 0 { "is-active" } else { "" }
                        >
                            "All"
                        </a>
                        <a
                            href="#"
                            class=move || if active_tab.get() == 1 { "is-active" } else { "" }
                        >
                            "Public"
                        </a>
                        <a
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
