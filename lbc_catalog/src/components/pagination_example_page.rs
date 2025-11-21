use leptos::prelude::{component, view, IntoView, ClassAttribute, ElementChild, Get, Set, Signal, create_signal};
use lbc::prelude::{Block, Content, HeaderSize, Subtitle, Title, Pagination, PaginationEllipsis, PaginationItem, PaginationItemType, Size, Alignment};
use std::sync::Arc;

#[component]
pub fn PaginationPage() -> impl IntoView {
    let (current_page, set_current_page) = create_signal(1usize);
    let total_pages = 5usize;

    let on_prev = {
        let set_current_page = set_current_page.clone();
        Arc::new(move || {
            let page = current_page.get();
            if page > 1 {
                set_current_page.set(page - 1);
            }
        })
    };
    let on_next = {
        let set_current_page = set_current_page.clone();
        Arc::new(move || {
            let page = current_page.get();
            if page < total_pages {
                set_current_page.set(page + 1);
            }
        })
    };

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Pagination"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic Pagination with Interaction"</Subtitle>
                <Pagination
                    previous_label="Previous"
                    next_label="Next"
                    alignment=Alignment::Centered
                    size=Size::Small
                    rounded=true
                    on_previous=on_prev.clone()
                    on_next=on_next.clone()
                >
                    <li>
                        <PaginationItem
                            item_type=PaginationItemType::Link
                            label="Page 1"
                            current=Signal::derive(move || current_page.get() == 1)
                            on_click={
                                let set_current_page = set_current_page.clone();
                                Arc::new(move || set_current_page.set(1))
                            }
                        >{"1"}</PaginationItem>
                    </li>
                    <li>
                        <PaginationItem
                            item_type=PaginationItemType::Link
                            label="Page 2"
                            current=Signal::derive(move || current_page.get() == 2)
                            on_click={
                                let set_current_page = set_current_page.clone();
                                Arc::new(move || set_current_page.set(2))
                            }
                        >{"2"}</PaginationItem>
                    </li>
                    <li>
                        <PaginationItem
                            item_type=PaginationItemType::Link
                            label="Page 3"
                            current=Signal::derive(move || current_page.get() == 3)
                            on_click={
                                let set_current_page = set_current_page.clone();
                                Arc::new(move || set_current_page.set(3))
                            }
                        >{"3"}</PaginationItem>
                    </li>
                    <li>
                        <PaginationItem
                            item_type=PaginationItemType::Link
                            label="Page 4"
                            current=Signal::derive(move || current_page.get() == 4)
                            on_click={
                                let set_current_page = set_current_page.clone();
                                Arc::new(move || set_current_page.set(4))
                            }
                        >{"4"}</PaginationItem>
                    </li>
                    <li>
                        <PaginationItem
                            item_type=PaginationItemType::Link
                            label="Page 5"
                            current=Signal::derive(move || current_page.get() == 5)
                            on_click={
                                Arc::new({
                                    let set_current_page = set_current_page.clone();
                                    move || set_current_page.set(5)
                                })
                            }
                        >{"5"}</PaginationItem>
                    </li>
                    <li><PaginationEllipsis character="…"/></li>
                </Pagination>
                <p class="help">
                    "Current page: "
                    {move || current_page.get().to_string()}
                </p>
            </Content>
        </Block>
    }
}
