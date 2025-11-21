/*!
Leptos version of Bulma Table element.

Bulma docs: https://bulma.io/documentation/elements/table/
*/

use leptos::prelude::{
    AnyView, Children, ClassAttribute, ElementChild, Get, GetUntracked, IntoAny, Signal, component, view,
};

/// An HTML table component.
///
/// https://bulma.io/documentation/elements/table/
#[component]
pub fn Table(
    /// Additional CSS classes to append to the base "table" class
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// Add borders to all the cells.
    #[prop(optional, into)]
    bordered: Signal<bool>,
    /// Add stripes to the table.
    #[prop(optional, into)]
    striped: Signal<bool>,
    /// Make the cells narrower.
    #[prop(optional, into)]
    narrow: Signal<bool>,
    /// Add a hover effect on each row.
    #[prop(optional, into)]
    hoverable: Signal<bool>,
    /// Make the table fullwidth.
    #[prop(optional, into)]
    fullwidth: Signal<bool>,
    /// Make the table scrollable, wrapping the table in a `div.table-container`.
    #[prop(optional, into)]
    scrollable: Signal<bool>,
    /// Child content to render inside the table
    children: Children,
) -> AnyView {
    let class_str = move || {
        let mut parts = vec!["table"];

        if bordered.get() {
            parts.push("is-bordered");
        }
        if striped.get() {
            parts.push("is-striped");
        }
        if narrow.get() {
            parts.push("is-narrow");
        }
        if hoverable.get() {
            parts.push("is-hoverable");
        }
        if fullwidth.get() {
            parts.push("is-fullwidth");
        }

        let mut result = parts.join(" ");

        if let Some(extra) = &classes {
            let extra_val = extra.get();
            if !extra_val.trim().is_empty() {
                result.push(' ');
                result.push_str(extra_val.trim());
            }
        }

        result
    };

    if scrollable.get_untracked() {
        view! {
            <div class="table-container">
                <table class=move || class_str()>
                    {children()}
                </table>
            </div>
        }
        .into_any()
    } else {
        view! {
            <table class=move || class_str()>
                {children()}
            </table>
        }
        .into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn table_renders_default() {
        let html = view! {
            <Table>
                <thead><tr><th>"Header"</th></tr></thead>
                <tbody><tr><td>"Data"</td></tr></tbody>
            </Table>
        }
        .to_html();
        assert!(
            html.contains(r#"class="table""#),
            "expected base 'table' class, got: {}",
            html
        );
        assert!(html.contains("<table"), "expected table tag");
        assert!(html.contains("Header"), "expected header content");
        assert!(html.contains("Data"), "expected body content");
    }

    #[test]
    fn table_with_bordered() {
        let html = view! {
            <Table bordered=true>
                <tbody><tr><td>"Cell"</td></tr></tbody>
            </Table>
        }
        .to_html();
        assert!(
            html.contains("is-bordered"),
            "expected is-bordered class, got: {}",
            html
        );
    }

    #[test]
    fn table_with_striped() {
        let html = view! {
            <Table striped=true>
                <tbody><tr><td>"Cell"</td></tr></tbody>
            </Table>
        }
        .to_html();
        assert!(
            html.contains("is-striped"),
            "expected is-striped class, got: {}",
            html
        );
    }

    #[test]
    fn table_with_narrow() {
        let html = view! {
            <Table narrow=true>
                <tbody><tr><td>"Cell"</td></tr></tbody>
            </Table>
        }
        .to_html();
        assert!(
            html.contains("is-narrow"),
            "expected is-narrow class, got: {}",
            html
        );
    }

    #[test]
    fn table_with_hoverable() {
        let html = view! {
            <Table hoverable=true>
                <tbody><tr><td>"Cell"</td></tr></tbody>
            </Table>
        }
        .to_html();
        assert!(
            html.contains("is-hoverable"),
            "expected is-hoverable class, got: {}",
            html
        );
    }

    #[test]
    fn table_with_fullwidth() {
        let html = view! {
            <Table fullwidth=true>
                <tbody><tr><td>"Cell"</td></tr></tbody>
            </Table>
        }
        .to_html();
        assert!(
            html.contains("is-fullwidth"),
            "expected is-fullwidth class, got: {}",
            html
        );
    }

    #[test]
    fn table_with_scrollable() {
        let html = view! {
            <Table scrollable=true>
                <tbody><tr><td>"Cell"</td></tr></tbody>
            </Table>
        }
        .to_html();
        assert!(
            html.contains(r#"class="table-container""#),
            "expected table-container wrapper, got: {}",
            html
        );
        assert!(html.contains("<table"), "expected table inside container");
    }

    #[test]
    fn table_with_custom_classes() {
        let html = view! {
            <Table classes="custom-class">
                <tbody><tr><td>"Cell"</td></tr></tbody>
            </Table>
        }
        .to_html();
        assert!(
            html.contains("custom-class"),
            "expected custom class, got: {}",
            html
        );
    }

    #[test]
    fn table_with_all_options() {
        let html = view! {
            <Table
                bordered=true
                striped=true
                narrow=true
                hoverable=true
                fullwidth=true
                classes="my-table"
            >
                <tbody><tr><td>"Cell"</td></tr></tbody>
            </Table>
        }
        .to_html();
        assert!(html.contains("is-bordered"), "expected is-bordered");
        assert!(html.contains("is-striped"), "expected is-striped");
        assert!(html.contains("is-narrow"), "expected is-narrow");
        assert!(html.contains("is-hoverable"), "expected is-hoverable");
        assert!(html.contains("is-fullwidth"), "expected is-fullwidth");
        assert!(html.contains("my-table"), "expected custom class");
    }

    #[test]
    fn table_scrollable_with_bordered() {
        let html = view! {
            <Table scrollable=true bordered=true>
                <tbody><tr><td>"Cell"</td></tr></tbody>
            </Table>
        }
        .to_html();
        assert!(
            html.contains(r#"class="table-container""#),
            "expected table-container for scrollable"
        );
        assert!(
            html.contains("is-bordered"),
            "expected is-bordered on table inside container"
        );
    }
}
