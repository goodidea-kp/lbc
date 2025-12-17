use lbc::prelude::{
    Alignment, Block, Breadcrumb, BreadcrumbSeparator, BreadcrumbSize, Buttons, Content,
    HeaderSize, Size, Title,
};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, signal, view};

#[component]
pub fn BreadcrumbPage() -> impl IntoView {
    #[allow(unused)]
    let (size, set_size) = signal::<Option<BreadcrumbSize>>(None);
    #[allow(unused)]
    let (alignment, set_alignment) = signal::<Option<Alignment>>(None);
    #[allow(unused)]
    let (separator, set_separator) = signal::<Option<BreadcrumbSeparator>>(None);

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Breadcrumb"</Title>

            <Buttons size=Size::Small>
                <button class="button" type="button">"Default"</button>
                <button class="button" type="button">"Small"</button>
                <button class="button" type="button">"Medium"</button>
                <button class="button" type="button">"Large"</button>
            </Buttons>

            <Buttons size=Size::Small>
                <button class="button" type="button">"Left"</button>
                <button class="button" type="button">"Centered"</button>
                <button class="button" type="button">"Right"</button>
            </Buttons>

            <Buttons size=Size::Small>
                <button class="button" type="button">"Default"</button>
                <button class="button" type="button">"Arrow"</button>
                <button class="button" type="button">"Bullet"</button>
                <button class="button" type="button">"Dot"</button>
                <button class="button" type="button">"Succeeds"</button>
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
