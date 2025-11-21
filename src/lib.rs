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

pub mod prelude {
    //! Commonly used re-exports for building UIs with LBC.
    //! Import this to bring the most frequently used components into scope.
    pub use crate::components::{
        AccordionItem, Accordions, Alignment, Breadcrumb, BreadcrumbSeparator, BreadcrumbSize,
        Calendar, Card, CardContent, CardFooter, CardHeader, CardImage, Dropdown, Menu, MenuLabel,
        MenuList, Message, MessageBody, MessageHeader, Modal, ModalCard, ModalCloserContext,
        ModalCloserProvider, Navbar, NavbarDivider, NavbarDropdown, NavbarFixed, NavbarItem,
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
