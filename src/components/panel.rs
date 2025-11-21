use leptos::prelude::{component, view, Children, ClassAttribute, ElementChild, Get, IntoAny, IntoView, OnAttribute, Signal};
use std::rc::Rc;

/// A composable panel, for compact controls.
/// https://bulma.io/documentation/components/panel/
#[component]
pub fn Panel(
    /// Extra classes added to the Bulma "panel" container.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional heading text for the panel.
    #[prop(optional, into)]
    heading: Option<Signal<String>>,

    /// Panel body content.
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || {
            let mut parts = vec!["panel".to_string()];
            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            parts.join(" ")
        }
    };

    let heading_node = {
        let heading = heading.clone();
        move || {
            heading.as_ref().map(|h| {
                view! { <p class="panel-heading">{h.get()}</p> }.into_any()
            })
        }
    };

    view! {
        <nav class=move || class()>
            {heading_node()}
            {children()}
        </nav>
    }
}

/// A container for the navigation tabs of a panel.
/// https://bulma.io/documentation/components/panel/
#[component]
pub fn PanelTabs(
    /// Tab anchors (<a>) to render inside this container.
    children: Children,
) -> impl IntoView {
    view! { <p class="panel-tabs">{children()}</p> }
}

/// An individual element of the panel.
/// https://bulma.io/documentation/components/panel/
#[component]
pub fn PanelBlock(
    /// The HTML tag to use for this component (div, a, button, p, span).
    #[prop(optional, into)]
    tag: Option<String>,

    /// Make this element highlighted/active.
    #[prop(optional, into)]
    active: Signal<bool>,

    /// Optional click handler for this block.
    #[prop(optional)]
    on_click: Option<Rc<dyn Fn()>>,

    /// Extra classes to apply to the panel-block element.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Child content for the block.
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let active = active.clone();
        move || {
            let mut parts = vec!["panel-block".to_string()];
            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if active.get() {
                parts.push("is-active".to_string());
            }
            parts.join(" ")
        }
    };

    view! {
        {
            let on_click_cb = {
                let on_click = on_click.clone();
                move |_| {
                    if let Some(cb) = &on_click {
                        cb();
                    }
                }
            };
            match tag.as_deref().unwrap_or("div") {
                "a" => view! { <a class=move || class() on:click=on_click_cb.clone()>{children()}</a> }.into_any(),
                "button" => view! { <button class=move || class() on:click=on_click_cb.clone()>{children()}</button> }.into_any(),
                "p" => view! { <p class=move || class() on:click=on_click_cb.clone()>{children()}</p> }.into_any(),
                "span" => view! { <span class=move || class() on:click=on_click_cb.clone()>{children()}</span> }.into_any(),
                _ => view! { <div class=move || class() on:click=on_click_cb.clone()>{children()}</div> }.into_any(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn panel_renders_default_with_heading_and_children() {
        let html = view! {
            <Panel heading="Heading">
                <div class="panel-block">"Child"</div>
            </Panel>
        }.to_html();

        assert!(html.contains(r#"class="panel""#), "expected base 'panel' class; got: {}", html);
        assert!(html.contains(r#"class="panel-heading""#) && html.contains("Heading"), "expected heading; got: {}", html);
        assert!(html.contains("Child"), "expected children rendered; got: {}", html);
    }

    #[test]
    fn panel_with_extra_classes() {
        let html = view! { <Panel classes="is-primary">"X"</Panel> }.to_html();
        assert!(html.contains(r#"class="panel is-primary""#) || html.contains(r#"class="panel is-primary ""#), "expected combined classes; got: {}", html);
    }

    #[test]
    fn panel_tabs_renders_container() {
        let html = view! { <PanelTabs><a>"All"</a></PanelTabs> }.to_html();
        assert!(html.contains(r#"class="panel-tabs""#), "expected 'panel-tabs' class; got: {}", html);
        assert!(html.contains("All"), "expected tab child; got: {}", html);
    }

    #[test]
    fn panel_block_default_tag_and_active() {
        let html = view! { <PanelBlock active=true>"Item"</PanelBlock> }.to_html();
        assert!(html.contains("<div") && html.contains("panel-block"), "expected default div with panel-block; got: {}", html);
        assert!(html.contains("is-active"), "expected is-active when active=true; got: {}", html);
    }

    #[test]
    fn panel_block_custom_tag_anchor() {
        let html = view! { <PanelBlock tag="a">"Link"</PanelBlock> }.to_html();
        assert!(html.contains("<a") && html.contains("panel-block"), "expected <a> tag with panel-block class; got: {}", html);
    }
}
