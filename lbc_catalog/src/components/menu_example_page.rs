use std::cell::Cell;
use std::rc::Rc;

use lbc::prelude::{Block, Content, HeaderSize, Menu, MenuLabel, MenuList, Title};
use leptos::html;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, GetUntracked, IntoAny, IntoView, NodeRef,
    NodeRefAttribute, Set, component, signal, view,
};

#[component]
pub fn MenuPage() -> impl IntoView {
    // Track the currently selected menu item and whether "Manage Your Team" submenu is open.
    let (selected, set_selected) = signal(String::from("Dashboard"));
    let (is_team_open, set_is_team_open) = signal(false);

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually on wasm32.
    let dashboard_ref: NodeRef<html::A> = NodeRef::new();
    let customers_ref: NodeRef<html::A> = NodeRef::new();
    let team_settings_ref: NodeRef<html::A> = NodeRef::new();
    let manage_team_ref: NodeRef<html::A> = NodeRef::new();
    let members_ref: NodeRef<html::A> = NodeRef::new();
    let plugins_ref: NodeRef<html::A> = NodeRef::new();
    let add_member_ref: NodeRef<html::A> = NodeRef::new();
    let invitations_ref: NodeRef<html::A> = NodeRef::new();
    let cloud_storage_ref: NodeRef<html::A> = NodeRef::new();
    let authentication_ref: NodeRef<html::A> = NodeRef::new();
    let payments_ref: NodeRef<html::A> = NodeRef::new();
    let transfers_ref: NodeRef<html::A> = NodeRef::new();
    let balance_ref: NodeRef<html::A> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::Event;

        fn attach_click_once(
            anchor_ref: NodeRef<html::A>,
            has_attached: Rc<Cell<bool>>,
            on_click: Rc<dyn Fn()>,
        ) {
            Effect::new(move |_| {
                if has_attached.get() {
                    return;
                }

                let Some(anchor_element) = anchor_ref.get() else {
                    return;
                };

                let on_click_for_event = on_click.clone();
                let click_closure: Closure<dyn FnMut(Event)> =
                    Closure::wrap(Box::new(move |event: Event| {
                        // Prevent navigation to "#" and keep SPA routing stable.
                        event.prevent_default();
                        (on_click_for_event)();
                    }));

                anchor_element
                    .add_event_listener_with_callback(
                        "click",
                        click_closure.as_ref().unchecked_ref(),
                    )
                    .ok();

                has_attached.set(true);
                click_closure.forget();
            });
        }

        let dashboard_attached = Rc::new(Cell::new(false));
        let customers_attached = Rc::new(Cell::new(false));
        let team_settings_attached = Rc::new(Cell::new(false));
        let manage_team_attached = Rc::new(Cell::new(false));
        let members_attached = Rc::new(Cell::new(false));
        let plugins_attached = Rc::new(Cell::new(false));
        let add_member_attached = Rc::new(Cell::new(false));
        let invitations_attached = Rc::new(Cell::new(false));
        let cloud_storage_attached = Rc::new(Cell::new(false));
        let authentication_attached = Rc::new(Cell::new(false));
        let payments_attached = Rc::new(Cell::new(false));
        let transfers_attached = Rc::new(Cell::new(false));
        let balance_attached = Rc::new(Cell::new(false));

        attach_click_once(
            dashboard_ref.clone(),
            dashboard_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Dashboard".to_string())
            }),
        );

        attach_click_once(
            customers_ref.clone(),
            customers_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Customers".to_string())
            }),
        );

        attach_click_once(
            team_settings_ref.clone(),
            team_settings_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Team Settings".to_string())
            }),
        );

        attach_click_once(
            manage_team_ref.clone(),
            manage_team_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                let set_is_team_open = set_is_team_open.clone();
                move || {
                    set_selected.set("Manage Your Team".to_string());
                    set_is_team_open.set(!is_team_open.get_untracked());
                }
            }),
        );

        attach_click_once(
            members_ref.clone(),
            members_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Members".to_string())
            }),
        );

        attach_click_once(
            plugins_ref.clone(),
            plugins_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Plugins".to_string())
            }),
        );

        attach_click_once(
            add_member_ref.clone(),
            add_member_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Add a member".to_string())
            }),
        );

        attach_click_once(
            invitations_ref.clone(),
            invitations_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Invitations".to_string())
            }),
        );

        attach_click_once(
            cloud_storage_ref.clone(),
            cloud_storage_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Cloud Storage Environment Settings".to_string())
            }),
        );

        attach_click_once(
            authentication_ref.clone(),
            authentication_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Authentication".to_string())
            }),
        );

        attach_click_once(
            payments_ref.clone(),
            payments_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Payments".to_string())
            }),
        );

        attach_click_once(
            transfers_ref.clone(),
            transfers_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Transfers".to_string())
            }),
        );

        attach_click_once(
            balance_ref.clone(),
            balance_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Balance".to_string())
            }),
        );
    }

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Menu"</Title>

            <Content>
                <Menu>
                    <MenuLabel text="General" />
                    <MenuList>
                        <li>
                            <a
                                node_ref=dashboard_ref
                                href="#"
                                class=move || if selected.get() == "Dashboard" { "is-active" } else { "" }
                            >
                                "Dashboard"
                            </a>
                        </li>
                        <li>
                            <a
                                node_ref=customers_ref
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
                                node_ref=team_settings_ref
                                href="#"
                                class=move || if selected.get() == "Team Settings" { "is-active" } else { "" }
                            >
                                "Team Settings"
                            </a>
                        </li>

                        <li>
                            <a
                                node_ref=manage_team_ref
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
                                                node_ref=members_ref
                                                href="#"
                                                class=move || if selected.get() == "Members" { "is-active" } else { "" }
                                            >
                                                "Members"
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                node_ref=plugins_ref
                                                href="#"
                                                class=move || if selected.get() == "Plugins" { "is-active" } else { "" }
                                            >
                                                "Plugins"
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                node_ref=add_member_ref
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
                                node_ref=invitations_ref
                                href="#"
                                class=move || if selected.get() == "Invitations" { "is-active" } else { "" }
                            >
                                "Invitations"
                            </a>
                        </li>
                        <li>
                            <a
                                node_ref=cloud_storage_ref
                                href="#"
                                class=move || if selected.get() == "Cloud Storage Environment Settings" { "is-active" } else { "" }
                            >
                                "Cloud Storage Environment Settings"
                            </a>
                        </li>
                        <li>
                            <a
                                node_ref=authentication_ref
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
                                node_ref=payments_ref
                                href="#"
                                class=move || if selected.get() == "Payments" { "is-active" } else { "" }
                            >
                                "Payments"
                            </a>
                        </li>
                        <li>
                            <a
                                node_ref=transfers_ref
                                href="#"
                                class=move || if selected.get() == "Transfers" { "is-active" } else { "" }
                            >
                                "Transfers"
                            </a>
                        </li>
                        <li>
                            <a
                                node_ref=balance_ref
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
