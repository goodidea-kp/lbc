use lbc::prelude::{
    Block, Buttons, Content, HeaderSize, Navbar, NavbarDivider, NavbarDropdown, NavbarItem, Title,
};
use leptos::prelude::{
    Callback, ClassAttribute, ElementChild, Get, IntoAny, IntoView, Set, Signal, component, signal,
    view,
};

#[component]
pub fn NavbarPage() -> impl IntoView {
    let (selected, set_selected) = signal(String::from("Home"));

    // Create click handlers inline to satisfy 'static closure requirements.

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Navbar"</Title>

            <Content>
                <p class="subtitle is-6">"Basic Navbar with Dropdown and Burger"</p>

                <Navbar
                    padded=true
                    spaced=true
                    transparent=false
                    navburger=true
                    brand=Box::new(|| view!{
                        <NavbarItem tag=lbc::components::navbar::NavbarItemTag::A href="https://bulma.io" target="_blank" rel="noopener">
                            <strong>"LBC"</strong>
                        </NavbarItem>
                    }.into_any())
                    start=Box::new(move || {
                        let selected_home = selected.clone();
                        let selected_docs = selected.clone();

                        let set_home = set_selected.clone();
                        let set_docs = set_selected.clone();
                        let set_about = set_selected.clone();
                        let set_jobs = set_selected.clone();
                        let set_contact = set_selected.clone();
                        let set_report = set_selected.clone();

                        view!{
                            <>
                                <NavbarItem tag=lbc::components::navbar::NavbarItemTag::A
                                            href="/"
                                            active=Signal::derive(move || selected_home.get() == "Home")
                                            on_click=Callback::new(move |_| set_home.set("Home".to_string()))
                                >
                                    "Home"
                                </NavbarItem>

                                <NavbarItem tag=lbc::components::navbar::NavbarItemTag::A
                                            href="/docs"
                                            active=Signal::derive(move || selected_docs.get() == "Docs")
                                            on_click=Callback::new(move |_| set_docs.set("Docs".to_string()))
                                >
                                    "Documentation"
                                </NavbarItem>

                                <NavbarDropdown
                                    navlink=Box::new(|| view!{ "More" }.into_any())
                                    hoverable=false
                                    boxed=true
                                >
                                    <NavbarItem tag=lbc::components::navbar::NavbarItemTag::A
                                                on_click=Callback::new(move |_| set_about.set("About".to_string()))
                                                auto_close=true
                                    >"About"</NavbarItem>
                                    <NavbarItem tag=lbc::components::navbar::NavbarItemTag::A
                                                on_click=Callback::new(move |_| set_jobs.set("Jobs".to_string()))
                                    >"Jobs"</NavbarItem>
                                    <NavbarItem tag=lbc::components::navbar::NavbarItemTag::A
                                                on_click=Callback::new(move |_| set_contact.set("Contact".to_string()))
                                    >"Contact"</NavbarItem>
                                    <NavbarDivider/>
                                    <NavbarItem tag=lbc::components::navbar::NavbarItemTag::A
                                                on_click=Callback::new(move |_| set_report.set("Report".to_string()))
                                    >"Report an issue"</NavbarItem>
                                </NavbarDropdown>
                            </>
                        }.into_any()
                    })
                    end=Box::new(|| view!{
                        <NavbarItem>
                            <Buttons>
                                <a class="button is-primary">
                                    <strong>"Sign up"</strong>
                                </a>
                                <a class="button is-light">"Log in"</a>
                            </Buttons>
                        </NavbarItem>
                    }.into_any())
                />

                <p class="help mt-3">
                    "Selected: " {move || selected.get()}
                </p>
            </Content>
        </Block>
    }
}
