use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, Get, IntoView, Signal, component, view,
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

/// Alignment options for Bulma tabs component.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    Centered,
    Right,
}

impl Alignment {
    fn bulma(self) -> &'static str {
        match self {
            Alignment::Centered => "is-centered",
            Alignment::Right => "is-right",
        }
    }
}

/// Simple responsive horizontal navigation tabs, with different styles.
/// https://bulma.io/documentation/components/tabs/
///
/// All class toggles are controlled via signals to match existing LBC patterns.
#[component]
pub fn Tabs(
    /// Child list items to render inside <ul>.
    children: Children,

    /// Extra classes to apply to the root "tabs" container.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// The alignment of this component.
    #[prop(optional)]
    alignment: Option<Alignment>,

    /// The size of this component.
    #[prop(optional)]
    size: Option<Size>,

    /// Add a more classic style with borders to this component.
    #[prop(optional, into)]
    boxed: Signal<bool>,

    /// Add the "radio button" style to the elements of this component.
    #[prop(optional, into)]
    toggle: Signal<bool>,

    /// Make the tab elements of this component rounded.
    #[prop(optional, into)]
    rounded: Signal<bool>,

    /// Make this component fullwidth.
    #[prop(optional, into)]
    fullwidth: Signal<bool>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let boxed = boxed.clone();
        let toggle = toggle.clone();
        let rounded = rounded.clone();
        let fullwidth = fullwidth.clone();

        move || {
            let mut parts = vec!["tabs".to_string()];

            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if let Some(align) = alignment {
                parts.push(align.bulma().to_string());
            }
            if let Some(sz) = size {
                parts.push(size_class(sz).to_string());
            }
            if boxed.get() {
                parts.push("is-boxed".to_string());
            }
            if toggle.get() {
                parts.push("is-toggle".to_string());
            }
            if rounded.get() {
                parts.push("is-rounded".to_string());
            }
            if fullwidth.get() {
                parts.push("is-fullwidth".to_string());
            }

            parts.join(" ")
        }
    };

    view! {
        <div class=move || class() data-testid=test_id>
            <ul>
                {children()}
            </ul>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Size;
    use leptos::prelude::RenderHtml;

    #[test]
    fn tabs_renders_base_class_and_structure() {
        let html = view! {
            <Tabs>
                <li class="is-active"><a>"One"</a></li>
                <li><a>"Two"</a></li>
            </Tabs>
        }
        .to_html();

        assert!(
            html.contains(r#"class="tabs""#),
            "expected base 'tabs' class; got: {}",
            html
        );
        assert!(
            html.contains("<ul"),
            "expected <ul> list wrapper; got: {}",
            html
        );
        assert!(
            html.contains("One") && html.contains("Two"),
            "expected children rendered; got: {}",
            html
        );
    }

    #[test]
    fn tabs_applies_alignment_and_size() {
        let html = view! {
            <Tabs alignment=Alignment::Centered size=Size::Small>
                <li><a>"X"</a></li>
            </Tabs>
        }
        .to_html();

        assert!(
            html.contains("is-centered"),
            "expected alignment class; got: {}",
            html
        );
        assert!(
            html.contains("is-small"),
            "expected size class; got: {}",
            html
        );
    }

    #[test]
    fn tabs_flag_classes() {
        let html = view! {
            <Tabs boxed=true toggle=true rounded=true fullwidth=true>
                <li><a>"X"</a></li>
            </Tabs>
        }
        .to_html();

        for cls in ["is-boxed", "is-toggle", "is-rounded", "is-fullwidth"] {
            assert!(
                html.contains(cls),
                "expected '{}' class present; got: {}",
                cls,
                html
            );
        }
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::Size;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn tabs_renders_test_id() {
        let html = view! {
            <Tabs
                classes="is-toggle"
                alignment=Alignment::Centered
                size=Size::Small
                boxed=true
                toggle=true
                rounded=true
                fullwidth=true
                test_id="tabs-test"
            >
                <li class="is-active"><a>"One"</a></li>
            </Tabs>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="tabs-test""#),
            "expected data-testid attribute on Tabs; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn tabs_no_test_id_when_not_provided() {
        let html = view! {
            <Tabs>
                <li class="is-active"><a>"One"</a></li>
            </Tabs>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on Tabs when not provided; got: {}",
            html
        );
    }
}
