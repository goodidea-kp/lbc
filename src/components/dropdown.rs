use leptos::prelude::{
    component, view, Children, ClassAttribute, ElementChild, Get, IntoAny, IntoView, OnAttribute,
    Signal, Set, StyleAttribute, AddAnyAttr, GlobalAttributes,
};
use crate::elements::button::Button;

/// A Bulma dropdown menu with a trigger button.
/// https://bulma.io/documentation/components/dropdown/
#[component]
pub fn Dropdown(
    /// Extra classes to apply to the root "dropdown" container.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Make this dropdown triggerable based on hover (CSS only).
    #[prop(optional, into)]
    hoverable: Signal<bool>,

    /// Extra classes to apply to the trigger Button.
    #[prop(optional, into)]
    button_classes: Signal<String>,

    /// Content placed inside the trigger Button.
    button: Children,

    /// Content placed inside the dropdown-content container.
    children: Children,
) -> impl IntoView {
    let (is_active, set_is_active) = leptos::prelude::signal(false);

    let class = {
        let classes = classes.clone();
        let hoverable = hoverable.clone();
        move || {
            let mut parts = vec!["dropdown".to_string()];
            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if hoverable.get() {
                parts.push("is-hoverable".to_string());
            }
            if is_active.get() {
                parts.push("is-active".to_string());
            }
            parts.join(" ")
        }
    };

    let open_click = move |_| {
        if !hoverable.get() {
            set_is_active.set(true);
        }
    };
    let close_click = move |_| set_is_active.set(false);

    view! {
        <div class=move || class()>
            {move || if is_active.get() && !hoverable.get() {
                // overlay to close when clicking outside
                view! {
                    <div on:click=close_click
                         style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"></div>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
            <div class="dropdown-trigger">
                <Button classes=button_classes.get() on:click=open_click>
                    {button()}
                </Button>
            </div>
            <div class="dropdown-menu" role="menu" style="position: relative; z-index: 20;">
                <div class="dropdown-content">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn dropdown_renders_base_structure() {
        let html = view! {
            <Dropdown button=Box::new(|| view!{ "Open" }.into_any())>
                <a class="dropdown-item">"Item 1"</a>
                <a class="dropdown-item">"Item 2"</a>
            </Dropdown>
        }.to_html();

        assert!(html.contains(r#"class="dropdown""#), "expected base 'dropdown' class; got: {}", html);
        assert!(html.contains("dropdown-menu") && html.contains("dropdown-content"), "expected dropdown structure; got: {}", html);
        assert!(html.contains("Open"), "expected button content rendered; got: {}", html);
        assert!(html.contains("Item 1") && html.contains("Item 2"), "expected children rendered; got: {}", html);
    }

    #[test]
    fn dropdown_hoverable_adds_class() {
        let html = view! {
            <Dropdown hoverable=true button=Box::new(|| view!{ "Btn" }.into_any())>
                <a class="dropdown-item">"X"</a>
            </Dropdown>
        }.to_html();

        assert!(html.contains("is-hoverable"), "expected is-hoverable class; got: {}", html);
    }
}
