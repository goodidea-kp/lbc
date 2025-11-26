/*!
Leptos version of Bulma Columns layout.

- Columns: responsive column container
- Column: single column with optional width and narrow modifier
- ColumnSize: maps to Bulma width helpers (e.g. is-half, is-one-third)

Follows existing crate patterns:
- optional props via #[prop(optional)] / #[prop(optional, into)]
- classes as Option<Signal<String>>
*/

use leptos::children::Children;
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, Get, Signal};
use leptos::{IntoView, component, view};

use crate::util::TestAttr;

/// Available widths for a `Column`, mapped to Bulma classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnSize {
    OneFifth,
    TwoFifths,
    ThreeFifths,
    FourFifths,
    OneQuarter,
    OneThird,
    Half,
    TwoThirds,
    ThreeQuarters,
    Full,
}

impl ColumnSize {
    /// Returns the Bulma CSS class corresponding to this `ColumnSize`.
    fn bulma(self) -> &'static str {
        match self {
            ColumnSize::OneFifth => "is-one-fifth",
            ColumnSize::TwoFifths => "is-two-fifths",
            ColumnSize::ThreeFifths => "is-three-fifths",
            ColumnSize::FourFifths => "is-four-fifths",
            ColumnSize::OneQuarter => "is-one-quarter",
            ColumnSize::OneThird => "is-one-third",
            ColumnSize::Half => "is-half",
            ColumnSize::TwoThirds => "is-two-thirds",
            ColumnSize::ThreeQuarters => "is-three-quarters",
            ColumnSize::Full => "is-full",
        }
    }
}

/// A responsive container for arranging content in columns.
///
/// https://bulma.io/documentation/columns/basics/
#[component]
pub fn Columns(
    #[prop(optional)] gapless: bool,
    #[prop(optional)] centered: bool,
    #[prop(optional)] vcentered: bool,
    #[prop(optional)] multiline: bool,
    #[prop(optional)] mobile: bool,
    #[prop(optional, into)] classes: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute) on the root <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (e.g., `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    children: Children,
) -> impl IntoView {
    let class = move || {
        let mut parts = vec!["columns"];
        if gapless {
            parts.push("is-gapless");
        }
        if centered {
            parts.push("is-centered");
        }
        if vcentered {
            parts.push("is-vcentered");
        }
        if multiline {
            parts.push("is-multiline");
        }
        if mobile {
            parts.push("is-mobile");
        }
        if let Some(extra) = &classes {
            let s = extra.get();
            if !s.is_empty() {
                return format!("{} {}", parts.join(" "), s);
            }
        }
        parts.join(" ")
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <div
            class=class
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {children()}
        </div>
    }
}

/// A single column inside a `Columns` container.
///
/// Use `size` to set a fixed width, and `narrow` to reduce the column's width to content.
#[component]
pub fn Column(
    #[prop(optional)] size: Option<ColumnSize>,
    #[prop(optional)] narrow: bool,
    #[prop(optional, into)] classes: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute) on the root <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (e.g., `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    children: Children,
) -> impl IntoView {
    let class = move || {
        let mut parts = vec!["column"];
        if let Some(s) = size {
            parts.push(s.bulma());
        }
        if narrow {
            parts.push("is-narrow");
        }
        if let Some(extra) = &classes {
            let s = extra.get();
            if !s.is_empty() {
                return format!("{} {}", parts.join(" "), s);
            }
        }
        parts.join(" ")
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <div
            class=class
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn columns_centered_multiline() {
        let html = view! {
            <Columns centered=true multiline=true>
                <Column><span>"A"</span></Column>
                <Column><span>"B"</span></Column>
            </Columns>
        }
        .to_html();
        assert!(html.contains(r#"class="columns is-centered is-multiline""#));
    }

    #[test]
    fn column_size_and_narrow() {
        let html = view! { <Column size=ColumnSize::Half narrow=true>"X"</Column> }.to_html();
        assert!(html.contains(r#"class="column is-half is-narrow""#));
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn columns_renders_test_attr_as_data_testid() {
        let html = view! {
            <Columns centered=true multiline=true test_attr=TestAttr::test_id("columns-test")>
                <Column>"A"</Column>
            </Columns>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="columns-test""#),
            "expected data-testid attribute on Columns; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn columns_no_test_attr_when_not_provided() {
        let html = view! {
            <Columns centered=true multiline=true>
                <Column>"A"</Column>
            </Columns>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute on Columns when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn column_renders_test_attr_as_data_testid() {
        let html = view! {
            <Column size=ColumnSize::Half narrow=true test_attr=TestAttr::test_id("column-test")>
                "X"
            </Column>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="column-test""#),
            "expected data-testid attribute on Column; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn column_no_test_attr_when_not_provided() {
        let html = view! {
            <Column size=ColumnSize::Half narrow=true>
                "X"
            </Column>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute on Column when not provided; got: {}",
            html
        );
    }
}
