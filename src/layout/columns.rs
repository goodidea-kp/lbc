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
use leptos::prelude::{ClassAttribute, ElementChild, Get, Signal};
use leptos::{IntoView, component, view};

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

    view! { <div class=class>{children()}</div> }
}

/// A single column inside a `Columns` container.
///
/// Use `size` to set a fixed width, and `narrow` to reduce the column's width to content.
#[component]
pub fn Column(
    #[prop(optional)] size: Option<ColumnSize>,
    #[prop(optional)] narrow: bool,
    #[prop(optional, into)] classes: Option<Signal<String>>,
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

    view! { <div class=class>{children()}</div> }
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
