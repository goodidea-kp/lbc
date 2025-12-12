use std::cell::Cell;
use std::rc::Rc;

use lbc::prelude::{
    Alignment, Block, Breadcrumb, BreadcrumbSeparator, BreadcrumbSize, Buttons, Content, HeaderSize,
    Size, Title,
};
use leptos::html;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, IntoView, NodeRef, NodeRefAttribute, Set, component,
    signal, view,
};

#[component]
pub fn BreadcrumbPage() -> impl IntoView {
    let (size, set_size) = signal::<Option<BreadcrumbSize>>(None);
    let (alignment, set_alignment) = signal::<Option<Alignment>>(None);
    let (separator, set_separator) = signal::<Option<BreadcrumbSeparator>>(None);

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually on wasm32.
    let size_default_ref: NodeRef<html::Button> = NodeRef::new();
    let size_small_ref: NodeRef<html::Button> = NodeRef::new();
    let size_medium_ref: NodeRef<html::Button> = NodeRef::new();
    let size_large_ref: NodeRef<html::Button> = NodeRef::new();

    let align_left_ref: NodeRef<html::Button> = NodeRef::new();
    let align_center_ref: NodeRef<html::Button> = NodeRef::new();
    let align_right_ref: NodeRef<html::Button> = NodeRef::new();

    let sep_default_ref: NodeRef<html::Button> = NodeRef::new();
    let sep_arrow_ref: NodeRef<html::Button> = NodeRef::new();
    let sep_bullet_ref: NodeRef<html::Button> = NodeRef::new();
    let sep_dot_ref: NodeRef<html::Button> = NodeRef::new();
    let sep_succeeds_ref: NodeRef<html::Button> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys::Event;

        fn attach_button_click_once(
            button_ref: NodeRef<html::Button>,
            has_attached: Rc<Cell<bool>>,
            on_click: Rc<dyn Fn()>,
        ) {
            Effect::new(move |_| {
                if has_attached.get() {
                    return;
                }

                let Some(button_element) = button_ref.get() else {
                    return;
                };

                let on_click_for_event = on_click.clone();
                let click_closure: Closure<dyn FnMut(Event)> =
                    Closure::wrap(Box::new(move |event: Event| {
                        event.prevent_default();
                        (on_click_for_event)();
                    }));

                button_element
                    .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
                    .ok();

                has_attached.set(true);
                click_closure.forget();
            });
        }

        attach_button_click_once(
            size_default_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_size = set_size.clone();
                move || set_size.set(None)
            }),
        );
        attach_button_click_once(
            size_small_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_size = set_size.clone();
                move || set_size.set(Some(BreadcrumbSize::Small))
            }),
        );
        attach_button_click_once(
            size_medium_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_size = set_size.clone();
                move || set_size.set(Some(BreadcrumbSize::Medium))
            }),
        );
        attach_button_click_once(
            size_large_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_size = set_size.clone();
                move || set_size.set(Some(BreadcrumbSize::Large))
            }),
        );

        attach_button_click_once(
            align_left_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_alignment = set_alignment.clone();
                move || set_alignment.set(None)
            }),
        );
        attach_button_click_once(
            align_center_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_alignment = set_alignment.clone();
                move || set_alignment.set(Some(Alignment::Centered))
            }),
        );
        attach_button_click_once(
            align_right_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_alignment = set_alignment.clone();
                move || set_alignment.set(Some(Alignment::Right))
            }),
        );

        attach_button_click_once(
            sep_default_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_separator = set_separator.clone();
                move || set_separator.set(None)
            }),
        );
        attach_button_click_once(
            sep_arrow_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_separator = set_separator.clone();
                move || set_separator.set(Some(BreadcrumbSeparator::Arrow))
            }),
        );
        attach_button_click_once(
            sep_bullet_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_separator = set_separator.clone();
                move || set_separator.set(Some(BreadcrumbSeparator::Bullet))
            }),
        );
        attach_button_click_once(
            sep_dot_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_separator = set_separator.clone();
                move || set_separator.set(Some(BreadcrumbSeparator::Dot))
            }),
        );
        attach_button_click_once(
            sep_succeeds_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_separator = set_separator.clone();
                move || set_separator.set(Some(BreadcrumbSeparator::Succeeds))
            }),
        );
    }

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Breadcrumb"</Title>

            <Buttons size=Size::Small>
                <button node_ref=size_default_ref class="button" type="button">"Default"</button>
                <button node_ref=size_small_ref class="button" type="button">"Small"</button>
                <button node_ref=size_medium_ref class="button" type="button">"Medium"</button>
                <button node_ref=size_large_ref class="button" type="button">"Large"</button>
            </Buttons>

            <Buttons size=Size::Small>
                <button node_ref=align_left_ref class="button" type="button">"Left"</button>
                <button node_ref=align_center_ref class="button" type="button">"Centered"</button>
                <button node_ref=align_right_ref class="button" type="button">"Right"</button>
            </Buttons>

            <Buttons size=Size::Small>
                <button node_ref=sep_default_ref class="button" type="button">"Default"</button>
                <button node_ref=sep_arrow_ref class="button" type="button">"Arrow"</button>
                <button node_ref=sep_bullet_ref class="button" type="button">"Bullet"</button>
                <button node_ref=sep_dot_ref class="button" type="button">"Dot"</button>
                <button node_ref=sep_succeeds_ref class="button" type="button">"Succeeds"</button>
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
