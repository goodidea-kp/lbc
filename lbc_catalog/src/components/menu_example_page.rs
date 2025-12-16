use lbc::prelude::{Block, Content, HeaderSize, Menu, MenuLabel, MenuList, Title};
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, IntoAny, IntoView, component, signal, view,
};

#[component]
pub fn MenuPage() -> impl IntoView {
    // Track the currently selected menu item and whether "Manage Your Team" submenu is open.
    let (selected, set_selected) = signal(String::from("Dashboard"));
    let (is_team_open, set_is_team_open) = signal(false);

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Menu"</Title>

            <Content>
                <Menu>
                    <MenuLabel text="General" />
                    <MenuList>
                        <li>
                            <a
                                href="#"
                                class=move || if selected.get() == "Dashboard" { "is-active" } else { "" }
                            >
                                "Dashboard"
                            </a>
                        </li>
                        <li>
                            <a
                                href="#"
                                class=move || if selected.get() == "Customers" { "is-active" } else { "" }
                            >
                                "Customers"
                            </a>
                        </li>
                    </MenuList>

                    <MenuLabel text="Administration" />
                    <MenuList>
                        <li>
                            <a
                                href="#"
                                class=move || if selected.get() == "Team Settings" { "is-active" } else { "" }
                            >
                                "Team Settings"
                            </a>
                        </li>

                        <li>
                            <a
                                href="#"
                                class=move || if selected.get() == "Manage Your Team" { "is-active" } else { "" }
                            >
                                {move || if is_team_open.get() { "▾ " } else { "▸ " }}
                                "Manage Your Team"
                            </a>
                            {move || if is_team_open.get() {
                                view! {
                                    <ul>
                                        <li>
                                            <a
                                                href="#"
                                                class=move || if selected.get() == "Members" { "is-active" } else { "" }
                                            >
                                                "Members"
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="#"
                                                class=move || if selected.get() == "Plugins" { "is-active" } else { "" }
                                            >
                                                "Plugins"
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="#"
                                                class=move || if selected.get() == "Add a member" { "is-active" } else { "" }
                                            >
                                                "Add a member"
                                            </a>
                                        </li>
                                    </ul>
                                }
                                .into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}
                        </li>

                        <li>
                            <a
                                href="#"
                                class=move || if selected.get() == "Invitations" { "is-active" } else { "" }
                            >
                                "Invitations"
                            </a>
                        </li>
                        <li>
                            <a
                                href="#"
                                class=move || if selected.get() == "Cloud Storage Environment Settings" { "is-active" } else { "" }
                            >
                                "Cloud Storage Environment Settings"
                            </a>
                        </li>
                        <li>
                            <a
                                href="#"
                                class=move || if selected.get() == "Authentication" { "is-active" } else { "" }
                            >
                                "Authentication"
                            </a>
                        </li>
                    </MenuList>

                    <MenuLabel text="Transactions" />
                    <MenuList>
                        <li>
                            <a
                                href="#"
                                class=move || if selected.get() == "Payments" { "is-active" } else { "" }
                            >
                                "Payments"
                            </a>
                        </li>
                        <li>
                            <a
                                href="#"
                                class=move || if selected.get() == "Transfers" { "is-active" } else { "" }
                            >
                                "Transfers"
                            </a>
                        </li>
                        <li>
                            <a
                                href="#"
                                class=move || if selected.get() == "Balance" { "is-active" } else { "" }
                            >
                                "Balance"
                            </a>
                        </li>
                    </MenuList>
                </Menu>

                <p class="help mt-3">"Selected: " {move || selected.get()}</p>
            </Content>
        </Block>
    }
}
