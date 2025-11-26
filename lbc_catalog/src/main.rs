use leptos::prelude::ElementChild;
use leptos::prelude::{AddAnyAttr, ClassAttribute, IntoView, component, mount_to_body, view};
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

mod columns_example_page;
mod components;
mod container_example_page;
mod elements;
mod footer_example_page;
mod form;
mod hero_example_page;
mod level_example_page;
mod media_example_page;
mod section_example_page;
mod tile_example_page;

use columns_example_page::ColumnsPage;
use components::accordion_example_page::AccordionPage;
use components::breadcrumb_example_page::BreadcrumbPage;
use components::calendar_example_page::CalendarPage;
use components::card_example_page::CardPage;
use components::dropdown_example_page::DropdownPage;
use components::menu_example_page::MenuPage;
use components::message_example_page::MessagePage;
use components::modal_example_page::ModalPage;
use components::navbar_example_page::NavbarPage;
use components::pagination_example_page::PaginationPage;
use components::panel_example_page::PanelPage;
use components::tabs_example_page::TabsPage;
use container_example_page::ContainerPage;

use elements::notification_example_page::NotificationPage;
use elements::progress_example_page::ProgressPage;
use elements::table_example_page::TablePage;
use elements::title_example_page::TitlePage;
use elements::{
    block_example_page::BlockPage, box_example_page::BoxPage, button_example_page::ButtonPage,
    colors_example_page::ColorsPage, content_example_page::ContentPage,
    delete_example_page::DeletePage, icon_example_page::IconPage, sizes_example_page::SizesPage,
    tag_example_page::TagPage,
};
use footer_example_page::FooterPage;
use form::form_autocomplete_example_page::FormAutoCompletePage;
use form::form_control_example_page::FormControlPage;
use form::form_example_page::FormCheckboxPage;
use form::form_field_example_page::FormFieldPage;
use form::form_file_example_page::FormFilePage;
use form::form_input_example_page::FormInputPage;
use form::form_radio_example_page::FormRadioPage;
use form::form_select_example_page::FormSelectPage;
use form::form_text_area_example_page::FormTextAreaPage;
use hero_example_page::HeroPage;
use level_example_page::LevelPage;
use media_example_page::MediaPage;
use section_example_page::SectionPage;
use tile_example_page::TilePage;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <section class="section">
                <div class="container">
                    <h1 class="title">"LBC Catalog"</h1>
                    <Nav/>
                    <Routes fallback=|| view! { <p>"Not Found"</p> }>
                        <Route path=() view=HomePage />

                        <Route path=path!("elements/button") view=ButtonPage />
                        <Route path=path!("elements/tag") view=TagPage />
                        <Route path=path!("elements/colors") view=ColorsPage />
                        <Route path=path!("elements/sizes") view=SizesPage />
                        <Route path=path!("elements/block") view=BlockPage />
                        <Route path=path!("elements/box") view=BoxPage />
                        <Route path=path!("elements/content") view=ContentPage />
                        <Route path=path!("elements/delete") view=DeletePage />
                        <Route path=path!("elements/icon") view=IconPage />
                        <Route path=path!("elements/notification") view=NotificationPage />
                        <Route path=path!("elements/progress") view=ProgressPage />
                        <Route path=path!("elements/title") view=TitlePage />
                        <Route path=path!("elements/table") view=TablePage />
                        <Route path=path!("components/tabs") view=TabsPage />
                        <Route path=path!("components/panel") view=PanelPage />
                        <Route path=path!("components/pagination") view=PaginationPage />
                        <Route path=path!("components/navbar") view=NavbarPage />
                        <Route path=path!("components/modal") view=ModalPage />
                        <Route path=path!("components/message") view=MessagePage />
                        <Route path=path!("components/menu") view=MenuPage />
                        <Route path=path!("components/dropdown") view=DropdownPage />
                        <Route path=path!("components/card") view=CardPage />
                        <Route path=path!("components/calendar") view=CalendarPage />
                        <Route path=path!("components/breadcrumb") view=BreadcrumbPage />
                        <Route path=path!("components/accordion") view=AccordionPage />

                        <Route path=path!("form/checkbox") view=FormCheckboxPage />
                        <Route path=path!("form/field") view=FormFieldPage />
                        <Route path=path!("form/control") view=FormControlPage />
                        <Route path=path!("form/file") view=FormFilePage />
                        <Route path=path!("form/input") view=FormInputPage />
                        <Route path=path!("form/radio") view=FormRadioPage />
                        <Route path=path!("form/select") view=FormSelectPage />
                        <Route path=path!("form/textarea") view=FormTextAreaPage />
                        <Route path=path!("form/autocomplete") view=FormAutoCompletePage />

                        <Route path=path!("layout/columns") view=ColumnsPage />
                        <Route path=path!("layout/container") view=ContainerPage />
                        <Route path=path!("layout/section") view=SectionPage />
                        <Route path=path!("layout/hero") view=HeroPage />
                        <Route path=path!("layout/level") view=LevelPage />
                        <Route path=path!("layout/tile") view=TilePage />
                        <Route path=path!("layout/media") view=MediaPage />
                        <Route path=path!("layout/footer") view=FooterPage />
                    </Routes>
                </div>
            </section>
        </Router>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <div class="block">
            <div class="buttons are-small">
                <A attr:class="button is-light" href="/">"Home"</A>
            </div>

            <h4 class="title is-6">"Elements"</h4>
            <div class="buttons are-small">
                <A attr:class="button is-link is-light" href="/elements/button">"Button"</A>
                <A attr:class="button is-info is-light" href="/elements/tag">"Tag"</A>
                <A attr:class="button is-danger is-light" href="/elements/colors">"Colors"</A>
                <A attr:class="button is-dark is-light" href="/elements/sizes">"Sizes"</A>
                <A attr:class="button is-success is-light" href="/elements/block">"Block"</A>
                <A attr:class="button is-success is-light" href="/elements/box">"Box"</A>
                <A attr:class="button is-success is-light" href="/elements/content">"Content"</A>
                <A attr:class="button is-danger is-light" href="/elements/delete">"Delete"</A>
                <A attr:class="button is-link is-light" href="/elements/icon">"Icon"</A>
                <A attr:class="button is-info is-light" href="/elements/notification">"Notification"</A>
                <A attr:class="button is-primary is-light" href="/elements/progress">"Progress"</A>
                <A attr:class="button is-warning is-light" href="/elements/title">"Title"</A>
                <A attr:class="button is-success is-light" href="/elements/table">"Table"</A>
            </div>

            <h4 class="title is-6">"Form"</h4>
            <div class="buttons are-small">
                <A attr:class="button is-link is-light" href="/form/checkbox">"Checkbox"</A>
                <A attr:class="button is-link is-light" href="/form/field">"Field"</A>
                <A attr:class="button is-link is-light" href="/form/control">"Control"</A>
                <A attr:class="button is-link is-light" href="/form/file">"File"</A>
                <A attr:class="button is-link is-light" href="/form/input">"Input"</A>
                <A attr:class="button is-link is-light" href="/form/radio">"Radio"</A>
                <A attr:class="button is-link is-light" href="/form/select">"Select"</A>
                <A attr:class="button is-link is-light" href="/form/textarea">"TextArea"</A>
                <A attr:class="button is-link is-light" href="/form/autocomplete">"AutoComplete"</A>
            </div>

            <h4 class="title is-6">"Components"</h4>
            <div class="buttons are-small">
                <A attr:class="button is-link is-light" href="/components/menu">"Menu"</A>
                <A attr:class="button is-link is-light" href="/components/dropdown">"Dropdown"</A>
                <A attr:class="button is-link is-light" href="/components/card">"Card"</A>
                <A attr:class="button is-link is-light" href="/components/calendar">"Calendar"</A>
                <A attr:class="button is-link is-light" href="/components/breadcrumb">"Breadcrumb"</A>
                <A attr:class="button is-link is-light" href="/components/accordion">"Accordion"</A>
                <A attr:class="button is-link is-light" href="/components/tabs">"Tabs"</A>
                <A attr:class="button is-link is-light" href="/components/panel">"Panel"</A>
                <A attr:class="button is-link is-light" href="/components/pagination">"Pagination"</A>
                <A attr:class="button is-link is-light" href="/components/navbar">"Navbar"</A>
                <A attr:class="button is-link is-light" href="/components/modal">"Modal"</A>
                <A attr:class="button is-link is-light" href="/components/message">"Message"</A>
            </div>

            <h4 class="title is-6">"Layout"</h4>
            <div class="buttons are-small">
                <A attr:class="button is-warning is-light" href="/layout/columns">"Columns"</A>
                <A attr:class="button is-primary is-light" href="/layout/container">"Container"</A>
                <A attr:class="button is-black is-light" href="/layout/section">"Section"</A>
                <A attr:class="button is-link is-light" href="/layout/hero">"Hero"</A>
                <A attr:class="button is-success is-light" href="/layout/level">"Level"</A>
                <A attr:class="button is-warning is-light" href="/layout/tile">"Tile"</A>
                <A attr:class="button is-info is-light" href="/layout/media">"Media"</A>
                <A attr:class="button is-black is-light" href="/layout/footer">"Footer"</A>
            </div>
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <>
            <h2 class="subtitle">"Primitives"</h2>
            <p>"Choose a component page from the navigation above."</p>
            <div class="buttons are-small mt-2">
                <A attr:class="button is-link is-light" href="/components/menu">"Go to Menu example"</A>
            </div>
        </>
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
