/*!
Accordion component: a Leptos wrapper around the bulma-accordion plugin.

Required static assets
- Add the bulma-accordion CSS into your HTML <head>:
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma-accordion@2.0.1/dist/css/bulma-accordion.min.css"/>

- Add the bulma-accordion JS so `bulmaAccordion` is available on window. Place this before your wasm bootstrap script
  (or ensure it loads before your app mounts):
  <script src="https://cdn.jsdelivr.net/npm/bulma-accordion@2.0.1/dist/js/bulma-accordion.min.js"></script>

Notes
- We attach the JS behavior on mount and detach on unmount.
- SSR tests only verify the rendered HTML structure.
*/

use leptos::prelude::{
    AriaAttributes, Children, ClassAttribute, ElementChild, Get, GlobalAttributes, IntoView,
    OnAttribute, Signal, component, view,
};

#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use leptos::web_sys::Element;

/// A single accordion item (article.accordion ...).
#[component]
pub fn AccordionItem(
    /// Title shown in the accordion header.
    #[prop(into)]
    title: Signal<String>,

    /// Initial open state (adds is-active).
    #[prop(optional, into)]
    open: Signal<bool>,

    /// Optional click handler invoked when header is clicked.
    #[prop(optional)]
    on_toggle: Option<std::rc::Rc<dyn Fn()>>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

    /// Body content of the accordion.
    children: Children,
) -> impl IntoView {
    let class = {
        let open = open.clone();
        move || {
            if open.get() {
                "accordion is-active".to_string()
            } else {
                "accordion".to_string()
            }
        }
    };

    let on_header_click = {
        let on_toggle = on_toggle.clone();
        move |_| {
            if let Some(cb) = &on_toggle {
                cb();
            }
        }
    };

    view! {
        <article class=move || class() data-testid=test_id>
            <div class="accordion-header">
                <p>{title.get()}</p>
                <button class="toggle" aria-label="toggle" on:click=on_header_click></button>
            </div>
            <div class="accordion-body">
                <div class="accordion-content">
                    {children()}
                </div>
            </div>
        </article>
    }
}

/// A container for accordion items. Attaches bulma-accordion on mount.
#[component]
pub fn Accordions(
    /// Unique DOM id for this accordion group (used to attach the JS widget).
    id: String,

    /// Extra classes for the root "accordions" container.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

    /// Items to render inside this accordion group.
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || {
            let extra = classes.get();
            if extra.trim().is_empty() {
                "accordions".to_string()
            } else {
                format!("accordions {}", extra)
            }
        }
    };

    // Attach/detach the JS plugin
    #[cfg(target_arch = "wasm32")]
    {
        let id_for_js = id.clone();
        leptos::prelude::Effect::new(move |_| {
            if let Some(element) = leptos::prelude::document().get_element_by_id(&id_for_js) {
                setup_accordion(&element);
            }
        });

        let id_cleanup = id.clone();
        leptos::prelude::on_cleanup(move || {
            detach_accordion(&JsValue::from(id_cleanup.as_str()));
        });
    }

    view! {
        <section id=id.clone() class=move || class() data-testid=test_id>
            {children()}
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
#[leptos::wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
let accordionInstances  = null;
export function setup_accordion(element) {
    if (accordionInstances === null) {
      accordionInstances = bulmaAccordion.attach('#' + element.id);
      return;
    }

    // Check if the accordion is already attached
    for (let i = 0; i < accordionInstances.length; i++) {
        if (accordionInstances[i].element && accordionInstances[i].element.id === element.id) {
            return;
        }
    }

    // If not attached, attach it
    let newAccordion = bulmaAccordion.attach('#' + element.id);
    accordionInstances.push(newAccordion);
}

export function detach_accordion(id) {
    if (!accordionInstances) return;
    for (let i = 0; i < accordionInstances.length; i++) {
        if (accordionInstances[i] && accordionInstances[i].element && accordionInstances[i].element.id === id) {
            accordionInstances[i].destroy();
            accordionInstances.splice(i, 1);
            break;
        }
    }

    if (accordionInstances.length === 0) {
        accordionInstances = null;
    }
}
"#)]
#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn setup_accordion(element: &Element);
    fn detach_accordion(id: &JsValue);
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn accordions_renders_container_and_item() {
        let html = view! {
            <Accordions id="acc1".to_string()>
                <AccordionItem title="One">
                    <p>"Body"</p>
                </AccordionItem>
            </Accordions>
        }
        .to_html();

        assert!(
            html.contains(r#"class="accordions""#),
            "expected accordions wrapper; got: {}",
            html
        );
        assert!(
            html.contains(r#"id="acc1""#),
            "expected id attribute; got: {}",
            html
        );
        assert!(
            html.contains(r#"class="accordion""#),
            "expected accordion item; got: {}",
            html
        );
        assert!(
            html.contains("One") && html.contains("Body"),
            "expected title and body; got: {}",
            html
        );
    }

    #[test]
    fn accordion_item_open_adds_is_active() {
        let html = view! {
            <AccordionItem title="T" open=true>
                <p>"B"</p>
            </AccordionItem>
        }
        .to_html();

        assert!(
            html.contains("is-active"),
            "expected is-active when open=true; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn accordions_renders_test_id() {
        let html = view! {
            <Accordions id="acc1".to_string() test_id="accordions-test">
                <AccordionItem title="One">
                    <p>"Body"</p>
                </AccordionItem>
            </Accordions>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="accordions-test""#),
            "expected data-testid attribute on Accordions; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn accordions_no_test_id_when_not_provided() {
        let html = view! {
            <Accordions id="acc1".to_string()>
                <AccordionItem title="One">
                    <p>"Body"</p>
                </AccordionItem>
            </Accordions>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on Accordions when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn accordion_item_renders_test_id() {
        let html = view! {
            <AccordionItem title="T" test_id="accordion-item-test">
                <p>"B"</p>
            </AccordionItem>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="accordion-item-test""#),
            "expected data-testid attribute on AccordionItem; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn accordion_item_no_test_id_when_not_provided() {
        let html = view! {
            <AccordionItem title="T">
                <p>"B"</p>
            </AccordionItem>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on AccordionItem when not provided; got: {}",
            html
        );
    }
}
