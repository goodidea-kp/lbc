use leptos::html;
use leptos::prelude::{
    AriaAttributes, Children, ClassAttribute, CustomAttribute, ElementChild, Get,
    GlobalAttributes, IntoView, NodeRef, NodeRefAttribute, Signal, component, view,
};

use crate::util::{Size, TestAttr};

fn size_class(size: Size) -> &'static str {
    match size {
        Size::Small => "is-small",
        Size::Normal => "is-normal",
        Size::Medium => "is-medium",
        Size::Large => "is-large",
    }
}

/// A pagination item type mapped to Bulma CSS classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaginationItemType {
    /// A pagination link for a specific page number.
    Link,
    /// A pagination button for the next page.
    Next,
    /// A pagination button for the previous page.
    Previous,
}

impl PaginationItemType {
    fn bulma(self) -> &'static str {
        match self {
            PaginationItemType::Link => "pagination-link",
            PaginationItemType::Next => "pagination-next",
            PaginationItemType::Previous => "pagination-previous",
        }
    }
}

/// A responsive, usable, and flexible pagination component.
/// https://bulma.io/documentation/components/pagination/
#[component]
pub fn Pagination(
    /// Pagination list items to render inside <ul class="pagination-list">.
    children: Children,

    /// Extra classes for the root "pagination" container.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// The size of this component.
    #[prop(optional)]
    size: Option<Size>,

    /// The alignment of this component (`is-centered`, `is-right`).
    ///
    /// Reuses Alignment enum from Tabs (Centered, Right).
    #[prop(optional)]
    alignment: Option<crate::components::tabs::Alignment>,

    /// Make the pagination elements rounded.
    #[prop(optional, into)]
    rounded: Signal<bool>,

    /// Label for the "previous" control.
    #[prop(into)]
    previous_label: Signal<String>,

    /// Label for the "next" control.
    #[prop(into)]
    next_label: Signal<String>,

    /// Click handler for the previous control.
    #[prop(optional)]
    on_previous: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,

    /// Click handler for the next control.
    #[prop(optional)]
    on_next: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,

    /// Optional test attribute for the root <nav>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let rounded = rounded.clone();
        move || {
            let mut parts = vec!["pagination".to_string()];

            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if let Some(sz) = size {
                parts.push(size_class(sz).to_string());
            }
            if let Some(align) = alignment {
                parts.push(match align {
                    crate::components::tabs::Alignment::Centered => "is-centered".to_string(),
                    crate::components::tabs::Alignment::Right => "is-right".to_string(),
                });
            }
            if rounded.get() {
                parts.push("is-rounded".to_string());
            }

            parts.join(" ")
        }
    };

    // Derive specific optional attributes that our macro can render.
    let (data_testid, data_cy) = match &test_attr {
        Some(ta) if ta.key == "data-testid" => (Some(ta.value.clone()), None),
        Some(ta) if ta.key == "data-cy" => (None, Some(ta.value.clone())),
        _ => (None, None),
    };

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listeners manually on wasm32.
    let previous_ref: NodeRef<html::A> = NodeRef::new();
    let next_ref: NodeRef<html::A> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::Event;

        let previous_attached = Rc::new(Cell::new(false));
        let next_attached = Rc::new(Cell::new(false));

        let previous_ref_for_effect = previous_ref.clone();
        let next_ref_for_effect = next_ref.clone();

        let on_previous_for_effect = on_previous.clone();
        let on_next_for_effect = on_next.clone();

        Effect::new(move |_| {
            if !previous_attached.get() {
                if let Some(previous_element) = previous_ref_for_effect.get() {
                    if let Some(callback) = on_previous_for_effect.clone() {
                        let click_closure: Closure<dyn FnMut(Event)> =
                            Closure::wrap(Box::new(move |event: Event| {
                                event.prevent_default();
                                callback();
                            }));

                        previous_element
                            .add_event_listener_with_callback(
                                "click",
                                click_closure.as_ref().unchecked_ref(),
                            )
                            .ok();

                        click_closure.forget();
                    }

                    previous_attached.set(true);
                }
            }

            if !next_attached.get() {
                if let Some(next_element) = next_ref_for_effect.get() {
                    if let Some(callback) = on_next_for_effect.clone() {
                        let click_closure: Closure<dyn FnMut(Event)> =
                            Closure::wrap(Box::new(move |event: Event| {
                                event.prevent_default();
                                callback();
                            }));

                        next_element
                            .add_event_listener_with_callback(
                                "click",
                                click_closure.as_ref().unchecked_ref(),
                            )
                            .ok();

                        click_closure.forget();
                    }

                    next_attached.set(true);
                }
            }
        });
    }

    view! {
        <nav
            class=move || class()
            role="navigation"
            aria-label="pagination"
            // Only support a known small set of custom attributes here.
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            <a
                node_ref=previous_ref
                class="pagination-previous"
                href="#"
            >
                {previous_label.get()}
            </a>
            <a
                node_ref=next_ref
                class="pagination-next"
                href="#"
            >
                {next_label.get()}
            </a>
            <ul class="pagination-list">
                {children()}
            </ul>
        </nav>
    }
}

/// A pagination element representing a link to a page number, the previous page or the next page.
/// https://bulma.io/documentation/components/pagination/
#[component]
pub fn PaginationItem(
    /// Inner content, usually a page number.
    children: Children,

    /// The pagination item type for this component.
    item_type: PaginationItemType,

    /// The aria-label to use for this element.
    #[prop(optional, into)]
    label: Signal<String>,

    /// Mark this item as the current page (adds "is-current").
    #[prop(optional, into)]
    current: Signal<bool>,

    /// Click handler for this item.
    #[prop(optional)]
    on_click: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,

    /// Optional test attribute for the <a>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let class = {
        let current = current.clone();
        move || {
            let mut parts = vec![item_type.bulma().to_string()];
            if current.get() {
                parts.push("is-current".to_string());
            }
            parts.join(" ")
        }
    };

    // Derive specific optional attributes that our macro can render.
    let (data_testid, data_cy) = match &test_attr {
        Some(ta) if ta.key == "data-testid" => (Some(ta.value.clone()), None),
        Some(ta) if ta.key == "data-cy" => (None, Some(ta.value.clone())),
        _ => (None, None),
    };

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:click` and attach click listener manually on wasm32.
    let item_ref: NodeRef<html::A> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::Event;

        let has_attached = Rc::new(Cell::new(false));
        let item_ref_for_effect = item_ref.clone();
        let on_click_for_effect = on_click.clone();

        Effect::new(move |_| {
            if has_attached.get() {
                return;
            }

            let Some(anchor_element) = item_ref_for_effect.get() else {
                return;
            };

            let Some(callback) = on_click_for_effect.clone() else {
                has_attached.set(true);
                return;
            };

            let click_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    event.prevent_default();
                    callback();
                }));

            anchor_element
                .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
                .ok();

            has_attached.set(true);
            click_closure.forget();
        });
    }

    view! {
        <a
            node_ref=item_ref
            class=move || class()
            aria-label=label.get()
            href="#"
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {children()}
        </a>
    }
}

/// A horizontal ellipsis for pagination range separators.
/// https://bulma.io/documentation/components/pagination/
#[component]
pub fn PaginationEllipsis(
    /// Character which will be used as ellipsis (default: "…")
    #[prop(into)]
    character: Signal<String>,
) -> impl IntoView {
    view! { <span class="pagination-ellipsis">{character.get()}</span> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn pagination_renders_base_and_list() {
        let html = view! {
            <Pagination previous_label="Prev" next_label="Next">
                <li><a class="pagination-link">"1"</a></li>
                <li><a class="pagination-link">"2"</a></li>
            </Pagination>
        }
        .to_html();

        assert!(
            html.contains(r#"class="pagination""#),
            "expected base 'pagination' class; got: {}",
            html
        );
        assert!(
            html.contains(r#"class="pagination-list""#),
            "expected pagination-list; got: {}",
            html
        );
        assert!(
            html.contains("Prev") && html.contains("Next"),
            "expected prev/next labels; got: {}",
            html
        );
    }

    #[test]
    fn pagination_item_types_and_current() {
        let html = view! {
            <>
                <PaginationItem item_type=PaginationItemType::Next label="Next">
                    {"Next"}
                </PaginationItem>
                <PaginationItem item_type=PaginationItemType::Previous label="Prev">
                    {"Prev"}
                </PaginationItem>
                <PaginationItem item_type=PaginationItemType::Link label="1" current=true>
                    {"1"}
                </PaginationItem>
            </>
        }
        .to_html();

        assert!(
            html.contains("pagination-next"),
            "expected pagination-next class; got: {}",
            html
        );
        assert!(
            html.contains("pagination-previous"),
            "expected pagination-previous class; got: {}",
            html
        );
        assert!(
            html.contains(r#"class="pagination-link is-current""#)
                || html.contains("pagination-link is-current "),
            "expected current class on link; got: {}",
            html
        );
    }

    #[test]
    fn pagination_ellipsis_renders() {
        let html = view! { <PaginationEllipsis character="..." /> }.to_html();
        assert!(
            html.contains("pagination-ellipsis") && html.contains("..."),
            "expected ellipsis; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::components::tabs::Alignment;
    use crate::util::{Size, TestAttr};
    use leptos::prelude::*;
    use std::sync::Arc;
    use wasm_bindgen_test::*;

    fn noop() -> Arc<dyn Fn() + Send + Sync> {
        Arc::new(|| {})
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn pagination_renders_test_attr_as_data_testid() {
        let html = view! {
            <Pagination
                previous_label="Prev"
                next_label="Next"
                classes="is-centered"
                size=Size::Small
                alignment=Alignment::Centered
                rounded=true
                on_previous=noop()
                on_next=noop()
                test_attr="pagination-test"
            >
                <li>
                    <PaginationItem item_type=PaginationItemType::Link label="1" current=true>
                        {"1"}
                    </PaginationItem>
                </li>
            </Pagination>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="pagination-test""#),
            "expected data-testid attribute on Pagination; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn pagination_no_test_attr_when_not_provided() {
        let html = view! {
            <Pagination previous_label="Prev" next_label="Next">
                <li>
                    <PaginationItem item_type=PaginationItemType::Link label="1" current=true>
                        {"1"}
                    </PaginationItem>
                </li>
            </Pagination>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on Pagination when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn pagination_item_renders_test_attr_as_data_testid() {
        let html = view! {
            <PaginationItem
                item_type=PaginationItemType::Link
                label="1"
                current=true
                on_click=noop()
                test_attr="pagination-item-test"
            >
                {"1"}
            </PaginationItem>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="pagination-item-test""#),
            "expected data-testid attribute on PaginationItem; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn pagination_item_no_test_attr_when_not_provided() {
        let html = view! {
            <PaginationItem
                item_type=PaginationItemType::Link
                label="1"
                current=true
                on_click=noop()
            >
                {"1"}
            </PaginationItem>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on PaginationItem when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn pagination_accepts_custom_test_attr_key() {
        let custom = TestAttr::new("data-cy", "pagination-cy");
        let html = view! {
            <Pagination
                previous_label="Prev"
                next_label="Next"
                test_attr=custom
            >
                <li>
                    <PaginationItem
                        item_type=PaginationItemType::Link
                        label="1"
                        current=true
                        test_attr=TestAttr::new("data-cy", "pagination-item-cy")
                    >
                        {"1"}
                    </PaginationItem>
                </li>
            </Pagination>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="pagination-cy""#),
            "expected custom data-cy attribute on Pagination; got: {}",
            html
        );
        assert!(
            html.contains(r#"data-cy="pagination-item-cy""#),
            "expected custom data-cy attribute on PaginationItem; got: {}",
            html
        );
    }
}
