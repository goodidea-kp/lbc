use leptos::prelude::{component, create_signal, view, ClassAttribute, ElementChild, Get, IntoAny, IntoView, OnAttribute, Set, Signal, AddAnyAttr};
use lbc::prelude::{Block, Button, Buttons, Content, HeaderSize, Size, Title, Breadcrumb, BreadcrumbSeparator, BreadcrumbSize, Alignment};

#[component]
pub fn BreadcrumbPage() -> impl IntoView {
    let (size, set_size) = create_signal::<Option<BreadcrumbSize>>(None);
    let (alignment, set_alignment) = create_signal::<Option<Alignment>>(None);
    let (separator, set_separator) = create_signal::<Option<BreadcrumbSeparator>>(None);

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Breadcrumb"</Title>

            <Buttons size=Size::Small>
                <Button on:click=move |_| set_size.set(None)>"Default"</Button>
                <Button on:click=move |_| set_size.set(Some(BreadcrumbSize::Small))>"Small"</Button>
                <Button on:click=move |_| set_size.set(Some(BreadcrumbSize::Medium))>"Medium"</Button>
                <Button on:click=move |_| set_size.set(Some(BreadcrumbSize::Large))>"Large"</Button>
            </Buttons>

            <Buttons size=Size::Small>
                <Button on:click=move |_| set_alignment.set(None)>"Left"</Button>
                <Button on:click=move |_| set_alignment.set(Some(Alignment::Centered))>"Centered"</Button>
                <Button on:click=move |_| set_alignment.set(Some(Alignment::Right))>"Right"</Button>
            </Buttons>

            <Buttons size=Size::Small>
                <Button on:click=move |_| set_separator.set(None)>"Default"</Button>
                <Button on:click=move |_| set_separator.set(Some(BreadcrumbSeparator::Arrow))>"Arrow"</Button>
                <Button on:click=move |_| set_separator.set(Some(BreadcrumbSeparator::Bullet))>"Bullet"</Button>
                <Button on:click=move |_| set_separator.set(Some(BreadcrumbSeparator::Dot))>"Dot"</Button>
                <Button on:click=move |_| set_separator.set(Some(BreadcrumbSeparator::Succeeds))>"Succeeds"</Button>
            </Buttons>

            <Content>
                <Breadcrumb
                    size=size
                    alignment=alignment
                    separator=separator
                >
                    <li><a href="#">"Bulma"</a></li>
                    <li><a href="#">"Documentation"</a></li>
                    <li><a href="#">"Components"</a></li>
                    <li class="is-active"><a href="#" aria-current="page">"Breadcrumb"</a></li>
                </Breadcrumb>
            </Content>
        </Block>
    }
}
