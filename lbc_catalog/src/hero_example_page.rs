/*!
Example page: Hero

AI Pair Programming Notes:
- Shows Bulma hero component in different colors and sizes using lbc Hero component.
*/

use lbc::prelude::{
    Block, Container, HeaderSize, Hero, HeroSize, Navbar, NavbarItem, Subtitle, Tabs, Title,
};
use leptos::prelude::{ClassAttribute, ElementChild, IntoAny, IntoView, component, view};

#[component]
pub fn HeroPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Hero"</Title>

            <Hero
                body=|| view! {
                    <Title>"Primary Hero"</Title>
                    <Subtitle>"Primary subtitle"</Subtitle>
                }
                classes="is-primary"
            />

            <Block classes="mt-4"><div></div></Block>

            <Hero
                body=|| view! {
                    <Title>"Info Hero"</Title>
                    <Subtitle>"Info subtitle"</Subtitle>
                }
                classes="is-info"
            />

            <Block classes="mt-4"><div></div></Block>

            <Hero
                body=|| view! {
                    <Title>"Large Hero with Gradient"</Title>
                    <Subtitle>"This hero has bold gradient"</Subtitle>
                }
                classes="is-success"
                size=HeroSize::Large
                bold=true
            />

            <Block classes="mt-4"><div></div></Block>

            <Hero
                body=|| view! {
                    <Title>"Hero with Head and Foot"</Title>
                    <Subtitle>"Complete structure"</Subtitle>
                }
                classes="is-warning"
                head={view! {
                    <Navbar
                        padded=true
                        brand=Box::new(|| view!{
                            <NavbarItem tag=lbc::components::navbar::NavbarItemTag::A href="#">
                                <strong>"Brand"</strong>
                            </NavbarItem>
                        }.into_any())
                    />
                }.into_any()}
                foot={view! {
                    <Container>
                        <Tabs boxed=true fullwidth=true>
                            <li class="is-active"><a>"Overview"</a></li>
                            <li><a>"Modifiers"</a></li>
                            <li><a>"Grid"</a></li>
                        </Tabs>
                    </Container>
                }.into_any()}
            />
        </Block>
    }
}
