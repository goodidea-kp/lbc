use leptos::prelude::{
    AriaAttributes, Children, ClassAttribute, CustomAttribute, ElementChild, Get, GlobalAttributes,
    IntoView, OnAttribute, Signal, component, view,
};

use crate::util::Size;

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

    /// Optional test identifier (renders as data-testid attribute) on the root <nav>.
    #[prop(optional, into)]
    test_id: Option<String>,
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

    let prev_click = {
        let on_previous = on_previous.clone();
        move |_| {
            if let Some(cb) = &on_previous {
                cb();
            }
        }
    };
    let next_click = {
        let on_next = on_next.clone();
        move |_| {
            if let Some(cb) = &on_next {
                cb();
            }
        }
    };

    view! {
        <nav class=move || class() role="navigation" aria-label="pagination" data-testid=test_id>
            <a class="pagination-previous" on:click=prev_click>{previous_label.get()}</a>
            <a class="pagination-next" on:click=next_click>{next_label.get()}</a>
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

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
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

    let click = {
        let on_click = on_click.clone();
        move |_| {
            if let Some(cb) = &on_click {
                cb();
            }
        }
    };

    view! {
        <a class=move || class() aria-label=label.get() on:click=click data-testid=test_id>
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
    use crate::util::Size;
    use leptos::prelude::*;
    use std::sync::Arc;
    use wasm_bindgen_test::*;

    fn noop() -> Arc<dyn Fn() + Send + Sync> {
        Arc::new(|| {})
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn pagination_renders_test_id() {
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
                test_id="pagination-test"
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
    fn pagination_no_test_id_when_not_provided() {
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
    fn pagination_item_renders_test_id() {
        let html = view! {
            <PaginationItem
                item_type=PaginationItemType::Link
                label="1"
                current=true
                on_click=Some(noop())
                test_id="pagination-item-test"
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
    fn pagination_item_no_test_id_when_not_provided() {
        let html = view! {
            <PaginationItem
                item_type=PaginationItemType::Link
                label="1"
                current=true
                on_click=Some(noop())
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
}
