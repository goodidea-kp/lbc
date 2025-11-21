use leptos::prelude::{
    component, create_signal, view, ClassAttribute, ElementChild, Get, IntoAny, IntoView, OnAttribute, Set,
};
use lbc::prelude::{Block, Content, HeaderSize, Title, Menu, MenuLabel, MenuList};

#[component]
pub fn MenuPage() -> impl IntoView {
    // Track the currently selected menu item and whether "Manage Your Team" submenu is open.
    let (selected, set_selected) = create_signal(String::from("Dashboard"));
    let (is_team_open, set_is_team_open) = create_signal(false);

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Menu"</Title>

            <Content>
                <Menu>
                    <MenuLabel text="General" />
                    <MenuList>
                        <li>
                            <a
                                class=move || if selected.get() == "Dashboard" { "is-active" } else { "" }
                                on:click=move |_| set_selected.set("Dashboard".to_string())
                            >
                                "Dashboard"
                            </a>
                        </li>
                        <li>
                            <a
                                class=move || if selected.get() == "Customers" { "is-active" } else { "" }
                                on:click=move |_| set_selected.set("Customers".to_string())
                            >
                                "Customers"
                            </a>
                        </li>
                    </MenuList>

                    <MenuLabel text="Administration" />
                    <MenuList>
                        <li>
                            <a
                                class=move || if selected.get() == "Team Settings" { "is-active" } else { "" }
                                on:click=move |_| set_selected.set("Team Settings".to_string())
                            >
                                "Team Settings"
                            </a>
                        </li>

                        <li>
                            <a
                                class=move || if selected.get() == "Manage Your Team" { "is-active" } else { "" }
                                on:click=move |_| {
                                    set_selected.set("Manage Your Team".to_string());
                                    set_is_team_open.set(!is_team_open.get());
                                }
                            >
                                {move || if is_team_open.get() { "▾ " } else { "▸ " }}
                                "Manage Your Team"
                            </a>
                            {move || if is_team_open.get() {
                                view! {
                                    <ul>
                                        <li>
                                            <a
                                                class=move || if selected.get() == "Members" { "is-active" } else { "" }
                                                on:click=move |_| set_selected.set("Members".to_string())
                                            >
                                                "Members"
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                class=move || if selected.get() == "Plugins" { "is-active" } else { "" }
                                                on:click=move |_| set_selected.set("Plugins".to_string())
                                            >
                                                "Plugins"
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                class=move || if selected.get() == "Add a member" { "is-active" } else { "" }
                                                on:click=move |_| set_selected.set("Add a member".to_string())
                                            >
                                                "Add a member"
                                            </a>
                                        </li>
                                    </ul>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}
                        </li>

                        <li>
                            <a
                                class=move || if selected.get() == "Invitations" { "is-active" } else { "" }
                                on:click=move |_| set_selected.set("Invitations".to_string())
                            >
                                "Invitations"
                            </a>
                        </li>
                        <li>
                            <a
                                class=move || if selected.get() == "Cloud Storage Environment Settings" { "is-active" } else { "" }
                                on:click=move |_| set_selected.set("Cloud Storage Environment Settings".to_string())
                            >
                                "Cloud Storage Environment Settings"
                            </a>
                        </li>
                        <li>
                            <a
                                class=move || if selected.get() == "Authentication" { "is-active" } else { "" }
                                on:click=move |_| set_selected.set("Authentication".to_string())
                            >
                                "Authentication"
                            </a>
                        </li>
                    </MenuList>

                    <MenuLabel text="Transactions" />
                    <MenuList>
                        <li>
                            <a
                                class=move || if selected.get() == "Payments" { "is-active" } else { "" }
                                on:click=move |_| set_selected.set("Payments".to_string())
                            >
                                "Payments"
                            </a>
                        </li>
                        <li>
                            <a
                                class=move || if selected.get() == "Transfers" { "is-active" } else { "" }
                                on:click=move |_| set_selected.set("Transfers".to_string())
                            >
                                "Transfers"
                            </a>
                        </li>
                        <li>
                            <a
                                class=move || if selected.get() == "Balance" { "is-active" } else { "" }
                                on:click=move |_| set_selected.set("Balance".to_string())
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
