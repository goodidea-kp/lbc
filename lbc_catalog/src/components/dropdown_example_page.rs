use std::cell::Cell;
use std::rc::Rc;

use lbc::prelude::{Block, Content, Dropdown, HeaderSize, Title};
use leptos::html;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, IntoAny, IntoView, NodeRef, NodeRefAttribute, Set,
    component, signal, view,
};

#[component]
pub fn DropdownPage() -> impl IntoView {
    let (selected, set_selected) = signal(String::from("-"));

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually on wasm32.
    let overview_ref: NodeRef<html::A> = NodeRef::new();
    let modifiers_ref: NodeRef<html::A> = NodeRef::new();
    let grid_ref: NodeRef<html::A> = NodeRef::new();
    let elements_ref: NodeRef<html::A> = NodeRef::new();
    let components_ref: NodeRef<html::A> = NodeRef::new();

    let first_ref: NodeRef<html::A> = NodeRef::new();
    let second_ref: NodeRef<html::A> = NodeRef::new();
    let third_ref: NodeRef<html::A> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
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
                    .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
                    .ok();

                has_attached.set(true);
                click_closure.forget();
            });
        }

        let overview_attached = Rc::new(Cell::new(false));
        let modifiers_attached = Rc::new(Cell::new(false));
        let grid_attached = Rc::new(Cell::new(false));
        let elements_attached = Rc::new(Cell::new(false));
        let components_attached = Rc::new(Cell::new(false));

        let first_attached = Rc::new(Cell::new(false));
        let second_attached = Rc::new(Cell::new(false));
        let third_attached = Rc::new(Cell::new(false));

        attach_click_once(
            overview_ref.clone(),
            overview_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Overview".to_string())
            }),
        );

        attach_click_once(
            modifiers_ref.clone(),
            modifiers_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Modifiers".to_string())
            }),
        );

        attach_click_once(
            grid_ref.clone(),
            grid_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Grid".to_string())
            }),
        );

        attach_click_once(
            elements_ref.clone(),
            elements_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Elements".to_string())
            }),
        );

        attach_click_once(
            components_ref.clone(),
            components_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Components".to_string())
            }),
        );

        attach_click_once(
            first_ref.clone(),
            first_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("First".to_string())
            }),
        );

        attach_click_once(
            second_ref.clone(),
            second_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Second".to_string())
            }),
        );

        attach_click_once(
            third_ref.clone(),
            third_attached,
            Rc::new({
                let set_selected = set_selected.clone();
                move || set_selected.set("Third".to_string())
            }),
        );
    }

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Dropdown"</Title>

            <Content>
                <p class="subtitle is-6">"Clickable Dropdown"</p>
                <Dropdown
                    classes=""
                    button_classes=""
                    button=Box::new(|| view! { "Open menu" }.into_any())
                >
                    <a node_ref=overview_ref class="dropdown-item" href="#">"Overview"</a>
                    <a node_ref=modifiers_ref class="dropdown-item" href="#">"Modifiers"</a>
                    <a node_ref=grid_ref class="dropdown-item" href="#">"Grid"</a>
                    <hr class="dropdown-divider"/>
                    <a node_ref=elements_ref class="dropdown-item" href="#">"Elements"</a>
                    <a node_ref=components_ref class="dropdown-item" href="#">"Components"</a>
                </Dropdown>

                <div class="mt-3"></div>

                <p class="subtitle is-6">"Hoverable Dropdown"</p>
                <Dropdown
                    hoverable=true
                    button_classes="is-link"
                    button=Box::new(|| view! {
                        <span>"Hover me"</span>
                    }.into_any())
                >
                    <a node_ref=first_ref class="dropdown-item" href="#">"First"</a>
                    <a node_ref=second_ref class="dropdown-item" href="#">"Second"</a>
                    <a node_ref=third_ref class="dropdown-item" href="#">"Third"</a>
                </Dropdown>

                <p class="help mt-3">"Selected: " {move || selected.get()}</p>
            </Content>
        </Block>
    }
}
