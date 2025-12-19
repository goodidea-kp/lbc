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
                <lbc::prelude::Button classes="is-primary">"Default"</lbc::prelude::Button>
                <lbc::prelude::Button classes="is-primary">"Small"</lbc::prelude::Button>
                <lbc::prelude::Button classes="is-primary">"Medium"</lbc::prelude::Button>
                <lbc::prelude::Button classes="is-primary">"Large"</lbc::prelude::Button>
            </Buttons>

            <Buttons size=Size::Small>
                <lbc::prelude::Button classes="is-primary">"Left"</lbc::prelude::Button>
                <lbc::prelude::Button classes="is-primary">"Centered"</lbc::prelude::Button>
                <lbc::prelude::Button classes="is-primary">"Right"</lbc::prelude::Button>
            </Buttons>

            <Buttons size=Size::Small>
                <lbc::prelude::Button classes="is-primary">"Default"</lbc::prelude::Button>
                <lbc::prelude::Button classes="is-primary">"Arrow"</lbc::prelude::Button>
                <lbc::prelude::Button classes="is-primary">"Bullet"</lbc::prelude::Button>
                <lbc::prelude::Button classes="is-primary">"Dot"</lbc::prelude::Button>
                <lbc::prelude::Button classes="is-primary">"Succeeds"</lbc::prelude::Button>
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
