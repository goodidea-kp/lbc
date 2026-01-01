#![allow(clippy::needless_doctest_main)]
//! LBC: Leptos + Bulma Components
//!
//! This crate provides a collection of Leptos components that render
//! Bulma CSS elements and layouts. Most modules mirror Bulma's
//! structure (elements, components, layout, form) and expose a
//! convenient prelude for common types.
//!
//! See each module and component for specific usage details and links
//! to the corresponding Bulma documentation.

pub mod components;
pub mod elements;
pub mod form;
pub mod layout;
pub mod util;

// Conditional logging macro; enable with `--features logging`
// When disabled, expands to a no-op.
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! lbc_log {
    ($($t:tt)*) => { ::leptos::logging::log!($($t)*) };
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! lbc_log {
    ($($t:tt)*) => {{ /* logging disabled */ }};
}

/// Debug logging that works without any crate features.
///
/// - On wasm32: logs to the browser console.
/// - On non-wasm: logs to stderr.
///
/// This is intentionally always enabled so debugging UI issues in `trunk serve`
/// doesn't require feature flags.
#[macro_export]
macro_rules! lbc_debug_log {
    ($($t:tt)*) => {{
        #[cfg(target_arch = "wasm32")]
        {
            ::leptos::web_sys::console::log_1(&::wasm_bindgen::JsValue::from_str(&format!($($t)*)));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            eprintln!($($t)*);
        }
    }};
}

pub mod prelude {
    //! Commonly used re-exports for building UIs with LBC.
    //! Import this to bring the most frequently used components into scope.
    pub use crate::components::{
        AccordionItem, Accordions, Alignment, Breadcrumb, BreadcrumbSeparator, BreadcrumbSize,
        Calendar, Card, CardContent, CardFooter, CardHeader, CardImage, Dropdown, Menu, MenuLabel,
        MenuList, Message, MessageBody, MessageHeader, Modal, ModalCard, ModalControllerContext,
        ModalControllerProvider, Navbar, NavbarDivider, NavbarDropdown, NavbarFixed, NavbarItem,
        NavbarMenuContext, Pagination, PaginationEllipsis, PaginationItem, PaginationItemType,
        Panel, PanelBlock, PanelTabs, Tabs,
    };
    pub use crate::elements::block::Block;
    pub use crate::elements::r#box::Box;
    pub use crate::elements::button::{Button, ButtonColor};
    pub use crate::elements::buttons::Buttons;
    pub use crate::elements::content::Content;
    pub use crate::elements::delete::Delete;
    pub use crate::elements::icon::{Icon, IconAlignment};
    pub use crate::elements::image::Image;
    pub use crate::elements::list::List;
    pub use crate::elements::notification::Notification;
    pub use crate::elements::progress::Progress;
    pub use crate::elements::table::Table;
    pub use crate::elements::tag::{Tag, TagColor};
    pub use crate::elements::tags::Tags;
    pub use crate::elements::title::{HeaderSize, Subtitle, Title};
    pub use crate::form::prelude::*;
    pub use crate::layout::columns::{Column, ColumnSize, Columns};
    pub use crate::layout::container::Container;
    pub use crate::layout::footer::Footer;
    pub use crate::layout::hero::{Hero, HeroSize};
    pub use crate::layout::level::{Level, LevelItem, LevelLeft, LevelRight};
    pub use crate::layout::media::{Media, MediaContent, MediaLeft, MediaRight};
    pub use crate::layout::section::{Section, SectionSize};
    pub use crate::layout::tile::{Tile, TileCtx, TileSize};
    pub use crate::util::Size;
}
