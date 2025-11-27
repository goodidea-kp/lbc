use leptos::prelude::{
    AriaAttributes, Children, ClassAttribute, CustomAttribute, ElementChild, Get, GetUntracked,
    GlobalAttributes, IntoAny, IntoView, OnAttribute, Set, Signal, StyleAttribute, component, view,
};
use std::rc::Rc;

use crate::util::TestAttr;

//// Context signal used to track global navbar menu open/closed state (burger/menu visibility).
pub type NavbarMenuContext = leptos::prelude::RwSignal<bool>;

/// The 2 possible fixed positions available for a navbar.
///
/// NOTE: for correct layout, your app shell should add
/// "has-navbar-fixed-top" or "has-navbar-fixed-bottom" to html/body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavbarFixed {
    Top,
    Bottom,
}

impl NavbarFixed {
    fn bulma(self) -> &'static str {
        match self {
            NavbarFixed::Top => "is-fixed-top",
            NavbarFixed::Bottom => "is-fixed-bottom",
        }
    }
}

/// A responsive horizontal navbar that can support images, links, and dropdowns.
/// https://bulma.io/documentation/components/navbar/
///
/// Slots:
/// - brand: left-side brand content; burger appended if `navburger=true`
/// - start: left part of the menu on desktop
/// - end: right part of the menu on desktop
#[component]
pub fn Navbar(
    /// Extra classes for the root "navbar".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Fix the navbar to the top or bottom.
    #[prop(optional)]
    fixed: Option<NavbarFixed>,

    /// Make the navbar transparent.
    #[prop(optional, into)]
    transparent: Signal<bool>,

    /// Add vertical spacing paddings to the navbar.
    #[prop(optional, into)]
    spaced: Signal<bool>,

    /// If true, wrap inner content in a Bulma "container".
    #[prop(optional, into)]
    padded: Signal<bool>,

    /// Render a burger menu for touch devices and toggle the menu on click.
    #[prop(optional, into)]
    navburger: Signal<bool>,

    /// Brand slot (left, always visible).
    #[prop(optional)]
    brand: Option<Children>,

    /// "navbar-start" slot (left part of the menu).
    #[prop(optional)]
    start: Option<Children>,

    /// "navbar-end" slot (right part of the menu).
    #[prop(optional)]
    end: Option<Children>,

    /// Optional test attribute (renders as data-* attribute) on the root <nav>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (e.g., `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let transparent = transparent.clone();
        let spaced = spaced.clone();
        move || {
            let mut parts = vec!["navbar".to_string()];
            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if let Some(fx) = fixed {
                parts.push(fx.bulma().to_string());
            }
            if transparent.get() {
                parts.push("is-transparent".to_string());
            }
            if spaced.get() {
                parts.push("is-spaced".to_string());
            }
            parts.join(" ")
        }
    };

    let is_menu_open = leptos::prelude::RwSignal::new(false);
    // Make menu open state available via context to descendants (e.g., items/dropdowns) or globally.
    leptos::prelude::provide_context::<NavbarMenuContext>(is_menu_open);

    // Pre-render slot children once to avoid moving FnOnce in reactive closures.
    let brand_view = brand.map(|children| children().into_any());
    let start_view = start.map(|children| children().into_any());
    let end_view = end.map(|children| children().into_any());

    let padded_initial = padded.get_untracked();

    // Derive specific optional attributes that our macro can render.
    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <nav
            class=move || class()
            role="navigation"
            aria-label="main navigation"
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {
                if padded_initial {
                    view! {
                        <div class="container">
                            <div class="navbar-brand">
                                {brand_view.unwrap_or_else(|| view! { <></> }.into_any())}
                                {move || if navburger.get() {
                                    let burger_class = move || if is_menu_open.get() { "navbar-burger is-active" } else { "navbar-burger" };
                                    view! {
                                        <a class=burger_class role="button" aria-label="menu" aria-expanded=move || if is_menu_open.get() { "true" } else { "false" }
                                           on:click=move |_| is_menu_open.set(!is_menu_open.get())>
                                            <span aria-hidden="true"></span>
                                            <span aria-hidden="true"></span>
                                            <span aria-hidden="true"></span>
                                            <span aria-hidden="true"></span>
                                        </a>
                                    }.into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }}
                            </div>
                            <div class=move || if is_menu_open.get() { "navbar-menu is-active" } else { "navbar-menu" }>
                                <div class="navbar-start">
                                    {start_view.unwrap_or_else(|| view! { <></> }.into_any())}
                                </div>
                                <div class="navbar-end">
                                    {end_view.unwrap_or_else(|| view! { <></> }.into_any())}
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <>
                            <div class="navbar-brand">
                                {brand_view.unwrap_or_else(|| view! { <></> }.into_any())}
                                {move || if navburger.get() {
                                    let burger_class = move || if is_menu_open.get() { "navbar-burger is-active" } else { "navbar-burger" };
                                    view! {
                                        <a class=burger_class role="button" aria-label="menu" aria-expanded=move || if is_menu_open.get() { "true" } else { "false" }
                                           on:click=move |_| is_menu_open.set(!is_menu_open.get())>
                                            <span aria-hidden="true"></span>
                                            <span aria-hidden="true"></span>
                                            <span aria-hidden="true"></span>
                                            <span aria-hidden="true"></span>
                                        </a>
                                    }.into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }}
                            </div>
                            <div class=move || if is_menu_open.get() { "navbar-menu is-active" } else { "navbar-menu" }>
                                <div class="navbar-start">
                                    {start_view.unwrap_or_else(|| view! { <></> }.into_any())}
                                </div>
                                <div class="navbar-end">
                                    {end_view.unwrap_or_else(|| view! { <></> }.into_any())}
                                </div>
                            </div>
                        </>
                    }.into_any()
                }
            }
        </nav>
    }
}

/// The two HTML tags allowed for a navbar-item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NavbarItemTag {
    A,
    Div,
}

/// A single element of the navbar.
/// https://bulma.io/documentation/components/navbar/
#[component]
pub fn NavbarItem(
    /// Child content of the navbar item.
    children: Children,

    /// Extra classes for the navbar item.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// The HTML tag to use for the item. Defaults to Div.
    #[prop(optional)]
    tag: Option<NavbarItemTag>,

    /// Optional click handler for this item.
    #[prop(optional)]
    on_click: Option<Rc<dyn Fn()>>,

    /// Add the `has-dropdown` class (used as a parent of a dropdown).
    #[prop(optional, into)]
    has_dropdown: Signal<bool>,

    /// Turn this into a full-width element.
    #[prop(optional, into)]
    expanded: Signal<bool>,

    /// Add bottom border on hover and show bottom border when active.
    #[prop(optional, into)]
    tab: Signal<bool>,

    /// Show active state (e.g., for tabs).
    #[prop(optional, into)]
    active: Signal<bool>,

    /// Attributes for anchor usage.
    #[prop(optional, into)]
    href: Signal<String>,
    #[prop(optional, into)] rel: Signal<String>,
    #[prop(optional, into)] target: Signal<String>,

    /// Optional test attribute (renders as data-* attribute) on the item element.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let has_dropdown = has_dropdown.clone();
        let expanded = expanded.clone();
        let tab = tab.clone();
        let active = active.clone();
        move || {
            let mut parts = vec!["navbar-item".to_string()];
            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if has_dropdown.get() {
                parts.push("has-dropdown".to_string());
            }
            if expanded.get() {
                parts.push("is-expanded".to_string());
            }
            if tab.get() {
                parts.push("is-tab".to_string());
            }
            if active.get() {
                parts.push("is-active".to_string());
            }
            parts.join(" ")
        }
    };

    let tag = tag.unwrap_or(NavbarItemTag::Div);

    let click_cb = {
        let on_click = on_click.clone();
        move |_| {
            if let Some(cb) = &on_click {
                cb();
            }
        }
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    match tag {
        NavbarItemTag::A => view! {
            <a class=move || class()
               href=href.get()
               rel=rel.get()
               target=target.get()
               on:click=click_cb.clone()
               attr:data-testid=move || data_testid.clone()
               attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </a>
        }
        .into_any(),
        NavbarItemTag::Div => view! {
            <div
                class=move || class()
                on:click=click_cb.clone()
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </div>
        }
        .into_any(),
    }
}

/// An element to display a horizontal rule in a navbar-dropdown.
#[component]
pub fn NavbarDivider(
    /// Extra classes to apply to the divider.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test attribute (renders as data-* attribute) on the <hr>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || {
            let extra = classes.get();
            if extra.trim().is_empty() {
                "navbar-divider".to_string()
            } else {
                format!("navbar-divider {}", extra)
            }
        }
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <hr
            class=move || class()
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        />
    }
}

/// A navbar dropdown menu: "navbar-item has-dropdown" parent + "navbar-dropdown".
#[component]
pub fn NavbarDropdown(
    /// Content of the dropdown (NavbarItem and NavbarDivider).
    children: Children,

    /// Extra classes for the parent container ("navbar-item has-dropdown ...").
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Content for the navbar-link trigger.
    navlink: Children,

    /// Make this dropdown triggerable based on hover (CSS only).
    #[prop(optional, into)]
    hoverable: Signal<bool>,

    /// Configure this dropdown as a dropup.
    #[prop(optional, into)]
    dropup: Signal<bool>,

    /// Position the dropdown to the right.
    #[prop(optional, into)]
    right: Signal<bool>,

    /// Remove the arrow from the navbar-link.
    #[prop(optional, into)]
    arrowless: Signal<bool>,

    /// Use the boxed style for the dropdown.
    #[prop(optional, into)]
    boxed: Signal<bool>,

    /// Optional test attribute (renders as data-* attribute) on the dropdown container.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let (is_active, set_is_active) = leptos::prelude::signal(false);

    let container_class = {
        let classes = classes.clone();
        let hoverable = hoverable.clone();
        let dropup = dropup.clone();
        move || {
            let mut parts = vec!["navbar-item".to_string(), "has-dropdown".to_string()];
            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if dropup.get() {
                parts.push("has-dropdown-up".to_string());
            }
            if hoverable.get() {
                parts.push("is-hoverable".to_string());
            }
            if is_active.get() && !hoverable.get() {
                parts.push("is-active".to_string());
            }
            parts.join(" ")
        }
    };

    let dropdown_class = {
        let right = right.clone();
        let boxed = boxed.clone();
        move || {
            let mut parts = vec!["navbar-dropdown".to_string()];
            if right.get() {
                parts.push("is-right".to_string());
            }
            if boxed.get() {
                parts.push("is-boxed".to_string());
            }
            parts.join(" ")
        }
    };

    let link_class = {
        let arrowless = arrowless.clone();
        move || {
            let mut parts = vec!["navbar-link".to_string()];
            if arrowless.get() {
                parts.push("is-arrowless".to_string());
            }
            parts.join(" ")
        }
    };

    let open_click = move |_| {
        if !hoverable.get() {
            set_is_active.set(true);
        }
    };
    let close_click = move |_| {
        if !hoverable.get() {
            set_is_active.set(false);
        }
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <div
            class=move || container_class()
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {move || if is_active.get() && !hoverable.get() {
                // overlay to close when clicking outside
                view! {
                    <div on:click=close_click
                         style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"></div>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
            <a class=move || link_class() on:click=open_click>{navlink()}</a>
            <div class=move || dropdown_class()>
                {children()}
            </div>
        </div>
    }
}
