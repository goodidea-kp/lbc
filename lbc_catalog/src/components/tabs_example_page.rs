use lbc::prelude::{Alignment, Size, Tabs};
use leptos::prelude::{
    ClassAttribute, ElementChild, Get, IntoView, OnAttribute, Set, component, signal, view,
};

#[component]
pub fn TabsPage() -> impl IntoView {
    // Track active selection for each tabs example.
    let (active_basic, set_active_basic) = signal(0usize);
    let (active_centered, set_active_centered) = signal(0usize);
    let (active_toggle, set_active_toggle) = signal(0usize);

    view! {
        <div class="block">
            <h3 class="title is-5">"Tabs"</h3>

            <div class="content">
                <p class="subtitle is-6">"Basic Tabs"</p>
                <Tabs>
                    <li
                        class=move || if active_basic.get() == 0 { "is-active" } else { "" }
                        on:click=move |_| set_active_basic.set(0)
                    >
                        <a>"Pictures"</a>
                    </li>
                    <li
                        class=move || if active_basic.get() == 1 { "is-active" } else { "" }
                        on:click=move |_| set_active_basic.set(1)
                    >
                        <a>"Music"</a>
                    </li>
                    <li
                        class=move || if active_basic.get() == 2 { "is-active" } else { "" }
                        on:click=move |_| set_active_basic.set(2)
                    >
                        <a>"Videos"</a>
                    </li>
                    <li
                        class=move || if active_basic.get() == 3 { "is-active" } else { "" }
                        on:click=move |_| set_active_basic.set(3)
                    >
                        <a>"Documents"</a>
                    </li>
                </Tabs>

                <p class="subtitle is-6">"Centered, Small, Boxed"</p>
                <Tabs alignment=Alignment::Centered size=Size::Small boxed=true>
                    <li
                        class=move || if active_centered.get() == 0 { "is-active" } else { "" }
                        on:click=move |_| set_active_centered.set(0)
                    >
                        <a>"Overview"</a>
                    </li>
                    <li
                        class=move || if active_centered.get() == 1 { "is-active" } else { "" }
                        on:click=move |_| set_active_centered.set(1)
                    >
                        <a>"Modifiers"</a>
                    </li>
                    <li
                        class=move || if active_centered.get() == 2 { "is-active" } else { "" }
                        on:click=move |_| set_active_centered.set(2)
                    >
                        <a>"Grid"</a>
                    </li>
                    <li
                        class=move || if active_centered.get() == 3 { "is-active" } else { "" }
                        on:click=move |_| set_active_centered.set(3)
                    >
                        <a>"Elements"</a>
                    </li>
                    <li
                        class=move || if active_centered.get() == 4 { "is-active" } else { "" }
                        on:click=move |_| set_active_centered.set(4)
                    >
                        <a>"Components"</a>
                    </li>
                </Tabs>

                <p class="subtitle is-6">"Toggle, Rounded, Fullwidth"</p>
                <Tabs toggle=true rounded=true fullwidth=true>
                    <li
                        class=move || if active_toggle.get() == 0 { "is-active" } else { "" }
                        on:click=move |_| set_active_toggle.set(0)
                    >
                        <a>"Yes"</a>
                    </li>
                    <li
                        class=move || if active_toggle.get() == 1 { "is-active" } else { "" }
                        on:click=move |_| set_active_toggle.set(1)
                    >
                        <a>"No"</a>
                    </li>
                </Tabs>
            </div>
        </div>
    }
}
