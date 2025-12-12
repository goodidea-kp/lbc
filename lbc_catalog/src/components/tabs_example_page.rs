use std::cell::Cell;
use std::rc::Rc;

use lbc::prelude::{Alignment, Size, Tabs};
use leptos::html;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, IntoView, NodeRef, NodeRefAttribute, Set, component,
    signal, view,
};

#[component]
pub fn TabsPage() -> impl IntoView {
    // Track active selection for each tabs example.
    let (active_basic, set_active_basic) = signal(0usize);
    let (active_centered, set_active_centered) = signal(0usize);
    let (active_toggle, set_active_toggle) = signal(0usize);

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually on wasm32.
    let basic_pictures_ref: NodeRef<html::Li> = NodeRef::new();
    let basic_music_ref: NodeRef<html::Li> = NodeRef::new();
    let basic_videos_ref: NodeRef<html::Li> = NodeRef::new();
    let basic_documents_ref: NodeRef<html::Li> = NodeRef::new();

    let centered_overview_ref: NodeRef<html::Li> = NodeRef::new();
    let centered_modifiers_ref: NodeRef<html::Li> = NodeRef::new();
    let centered_grid_ref: NodeRef<html::Li> = NodeRef::new();
    let centered_elements_ref: NodeRef<html::Li> = NodeRef::new();
    let centered_components_ref: NodeRef<html::Li> = NodeRef::new();

    let toggle_yes_ref: NodeRef<html::Li> = NodeRef::new();
    let toggle_no_ref: NodeRef<html::Li> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::Event;

        fn attach_li_click_once(
            li_ref: NodeRef<html::Li>,
            has_attached: Rc<Cell<bool>>,
            on_click: Rc<dyn Fn()>,
        ) {
            Effect::new(move |_| {
                if has_attached.get() {
                    return;
                }

                let Some(li_element) = li_ref.get() else {
                    return;
                };

                let on_click_for_event = on_click.clone();
                let click_closure: Closure<dyn FnMut(Event)> =
                    Closure::wrap(Box::new(move |event: Event| {
                        // Prevent any default navigation if the click originated from an <a href="#">
                        event.prevent_default();
                        (on_click_for_event)();
                    }));

                li_element
                    .add_event_listener_with_callback(
                        "click",
                        click_closure.as_ref().unchecked_ref(),
                    )
                    .ok();

                has_attached.set(true);
                click_closure.forget();
            });
        }

        attach_li_click_once(
            basic_pictures_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_basic = set_active_basic.clone();
                move || set_active_basic.set(0)
            }),
        );
        attach_li_click_once(
            basic_music_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_basic = set_active_basic.clone();
                move || set_active_basic.set(1)
            }),
        );
        attach_li_click_once(
            basic_videos_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_basic = set_active_basic.clone();
                move || set_active_basic.set(2)
            }),
        );
        attach_li_click_once(
            basic_documents_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_basic = set_active_basic.clone();
                move || set_active_basic.set(3)
            }),
        );

        attach_li_click_once(
            centered_overview_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_centered = set_active_centered.clone();
                move || set_active_centered.set(0)
            }),
        );
        attach_li_click_once(
            centered_modifiers_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_centered = set_active_centered.clone();
                move || set_active_centered.set(1)
            }),
        );
        attach_li_click_once(
            centered_grid_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_centered = set_active_centered.clone();
                move || set_active_centered.set(2)
            }),
        );
        attach_li_click_once(
            centered_elements_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_centered = set_active_centered.clone();
                move || set_active_centered.set(3)
            }),
        );
        attach_li_click_once(
            centered_components_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_centered = set_active_centered.clone();
                move || set_active_centered.set(4)
            }),
        );

        attach_li_click_once(
            toggle_yes_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_toggle = set_active_toggle.clone();
                move || set_active_toggle.set(0)
            }),
        );
        attach_li_click_once(
            toggle_no_ref.clone(),
            Rc::new(Cell::new(false)),
            Rc::new({
                let set_active_toggle = set_active_toggle.clone();
                move || set_active_toggle.set(1)
            }),
        );
    }

    view! {
        <div class="block">
            <h3 class="title is-5">"Tabs"</h3>

            <div class="content">
                <p class="subtitle is-6">"Basic Tabs"</p>
                <Tabs>
                    <li
                        node_ref=basic_pictures_ref
                        class=move || if active_basic.get() == 0 { "is-active" } else { "" }
                    >
                        <a href="#">"Pictures"</a>
                    </li>
                    <li
                        node_ref=basic_music_ref
                        class=move || if active_basic.get() == 1 { "is-active" } else { "" }
                    >
                        <a href="#">"Music"</a>
                    </li>
                    <li
                        node_ref=basic_videos_ref
                        class=move || if active_basic.get() == 2 { "is-active" } else { "" }
                    >
                        <a href="#">"Videos"</a>
                    </li>
                    <li
                        node_ref=basic_documents_ref
                        class=move || if active_basic.get() == 3 { "is-active" } else { "" }
                    >
                        <a href="#">"Documents"</a>
                    </li>
                </Tabs>

                <p class="subtitle is-6">"Centered, Small, Boxed"</p>
                <Tabs alignment=Alignment::Centered size=Size::Small boxed=true>
                    <li
                        node_ref=centered_overview_ref
                        class=move || if active_centered.get() == 0 { "is-active" } else { "" }
                    >
                        <a href="#">"Overview"</a>
                    </li>
                    <li
                        node_ref=centered_modifiers_ref
                        class=move || if active_centered.get() == 1 { "is-active" } else { "" }
                    >
                        <a href="#">"Modifiers"</a>
                    </li>
                    <li
                        node_ref=centered_grid_ref
                        class=move || if active_centered.get() == 2 { "is-active" } else { "" }
                    >
                        <a href="#">"Grid"</a>
                    </li>
                    <li
                        node_ref=centered_elements_ref
                        class=move || if active_centered.get() == 3 { "is-active" } else { "" }
                    >
                        <a href="#">"Elements"</a>
                    </li>
                    <li
                        node_ref=centered_components_ref
                        class=move || if active_centered.get() == 4 { "is-active" } else { "" }
                    >
                        <a href="#">"Components"</a>
                    </li>
                </Tabs>

                <p class="subtitle is-6">"Toggle, Rounded, Fullwidth"</p>
                <Tabs toggle=true rounded=true fullwidth=true>
                    <li
                        node_ref=toggle_yes_ref
                        class=move || if active_toggle.get() == 0 { "is-active" } else { "" }
                    >
                        <a href="#">"Yes"</a>
                    </li>
                    <li
                        node_ref=toggle_no_ref
                        class=move || if active_toggle.get() == 1 { "is-active" } else { "" }
                    >
                        <a href="#">"No"</a>
                    </li>
                </Tabs>
            </div>
        </div>
    }
}
