use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, Get, IntoAny, IntoView, OnAttribute,
    Signal, component, view,
};
use std::rc::Rc;

use crate::util::TestAttr;

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

    /// Optional test attribute for the root <nav>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (e.g., `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

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
            heading
                .as_ref()
                .map(|h| view! { <p class="panel-heading">{h.get()}</p> }.into_any())
        }
    };

    // Derive specific optional attributes that our macro can render.
    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <nav
            class=move || class()
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
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

    /// Optional test attribute for the <p>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <p
            class="panel-tabs"
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {children()}
        </p>
    }
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

    /// Optional test attribute for the rendered element.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

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

    // Derive specific optional attributes that our macro can render.
    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
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
                "a" => view! {
                    <a
                        class=move || class()
                        on:click=on_click_cb.clone()
                        attr:data-testid=move || data_testid.clone()
                        attr:data-cy=move || data_cy.clone()
                    >
                        {children()}
                    </a>
                }.into_any(),
                "button" => view! {
                    <button
                        class=move || class()
                        on:click=on_click_cb.clone()
                        attr:data-testid=move || data_testid.clone()
                        attr:data-cy=move || data_cy.clone()
                    >
                        {children()}
                    </button>
                }.into_any(),
                "p" => view! {
                    <p
                        class=move || class()
                        on:click=on_click_cb.clone()
                        attr:data-testid=move || data_testid.clone()
                        attr:data-cy=move || data_cy.clone()
                    >
                        {children()}
                    </p>
                }.into_any(),
                "span" => view! {
                    <span
                        class=move || class()
                        on:click=on_click_cb.clone()
                        attr:data-testid=move || data_testid.clone()
                        attr:data-cy=move || data_cy.clone()
                    >
                        {children()}
                    </span>
                }.into_any(),
                _ => view! {
                    <div
                        class=move || class()
                        on:click=on_click_cb.clone()
                        attr:data-testid=move || data_testid.clone()
                        attr:data-cy=move || data_cy.clone()
                    >
                        {children()}
                    </div>
                }.into_any(),
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
        }
        .to_html();

        assert!(
            html.contains(r#"class="panel""#),
            "expected base 'panel' class; got: {}",
            html
        );
        assert!(
            html.contains(r#"class="panel-heading""#) && html.contains("Heading"),
            "expected heading; got: {}",
            html
        );
        assert!(
            html.contains("Child"),
            "expected children rendered; got: {}",
            html
        );
    }

    #[test]
    fn panel_with_extra_classes() {
        let html = view! { <Panel classes="is-primary">"X"</Panel> }.to_html();
        assert!(
            html.contains(r#"class="panel is-primary""#)
                || html.contains(r#"class="panel is-primary ""#),
            "expected combined classes; got: {}",
            html
        );
    }

    #[test]
    fn panel_tabs_renders_container() {
        let html = view! { <PanelTabs><a>"All"</a></PanelTabs> }.to_html();
        assert!(
            html.contains(r#"class="panel-tabs""#),
            "expected 'panel-tabs' class; got: {}",
            html
        );
        assert!(html.contains("All"), "expected tab child; got: {}", html);
    }

    #[test]
    fn panel_block_default_tag_and_active() {
        let html = view! { <PanelBlock active=true>"Item"</PanelBlock> }.to_html();
        assert!(
            html.contains("<div") && html.contains("panel-block"),
            "expected default div with panel-block; got: {}",
            html
        );
        assert!(
            html.contains("is-active"),
            "expected is-active when active=true; got: {}",
            html
        );
    }

    #[test]
    fn panel_block_custom_tag_anchor() {
        let html = view! { <PanelBlock tag="a">"Link"</PanelBlock> }.to_html();
        assert!(
            html.contains("<a") && html.contains("panel-block"),
            "expected <a> tag with panel-block class; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use std::rc::Rc;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn noop() -> Rc<dyn Fn()> {
        Rc::new(|| {})
    }

    #[wasm_bindgen_test]
    fn panel_renders_test_attr_as_data_testid() {
        let html = view! {
            <Panel classes="is-primary" heading="Heading" test_attr="panel-test">
                <div class="panel-block">"Child"</div>
            </Panel>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="panel-test""#),
            "expected data-testid attribute on Panel; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn panel_no_test_attr_when_not_provided() {
        let html = view! {
            <Panel heading="Heading">
                <div class="panel-block">"Child"</div>
            </Panel>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on Panel when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn panel_tabs_renders_test_attr_as_data_testid() {
        let html = view! {
            <PanelTabs test_attr="panel-tabs-test">
                <a>"All"</a>
            </PanelTabs>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="panel-tabs-test""#),
            "expected data-testid attribute on PanelTabs; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn panel_block_renders_test_attr_as_data_testid() {
        let html = view! {
            <PanelBlock active=true on_click=noop() test_attr="panel-block-test">
                "Item"
            </PanelBlock>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="panel-block-test""#),
            "expected data-testid attribute on PanelBlock; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn panel_block_no_test_attr_when_not_provided() {
        let html = view! {
            <PanelBlock active=true on_click=noop()>
                "Item"
            </PanelBlock>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on PanelBlock when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn panel_accepts_custom_test_attr_key() {
        let html = view! {
            <Panel
                classes="is-primary"
                heading="Heading"
                test_attr=TestAttr::new("data-cy", "panel-cy")
            >
                <div class="panel-block">"Child"</div>
            </Panel>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="panel-cy""#),
            "expected custom data-cy attribute on Panel; got: {}",
            html
        );
    }
}
