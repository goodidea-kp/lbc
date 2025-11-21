use crate::components::tabs::Alignment;
use leptos::prelude::{
    AriaAttributes, Children, ClassAttribute, ElementChild, Get, IntoView, Signal, component, view,
};

/// The 3 sizes available for a breadcrumb.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreadcrumbSize {
    Small,
    Medium,
    Large,
}

impl BreadcrumbSize {
    fn bulma(self) -> &'static str {
        match self {
            BreadcrumbSize::Small => "is-small",
            BreadcrumbSize::Medium => "is-medium",
            BreadcrumbSize::Large => "is-large",
        }
    }
}

/// The 4 additional separators for a breadcrumb.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreadcrumbSeparator {
    Arrow,
    Bullet,
    Dot,
    Succeeds,
}

impl BreadcrumbSeparator {
    fn bulma(self) -> &'static str {
        match self {
            BreadcrumbSeparator::Arrow => "has-arrow-separator",
            BreadcrumbSeparator::Bullet => "has-bullet-separator",
            BreadcrumbSeparator::Dot => "has-dot-separator",
            BreadcrumbSeparator::Succeeds => "has-succeeds-separator",
        }
    }
}

/// A simple breadcrumb component to improve your navigation experience.
///
/// https://bulma.io/documentation/components/breadcrumb/
#[component]
pub fn Breadcrumb(
    /// The `li` child elements of this breadcrumb.
    children: Children,

    /// Extra classes to apply to the root "breadcrumb" container.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// The size of this component.
    #[prop(optional, into)]
    size: Signal<Option<BreadcrumbSize>>,

    /// The alignment of this component.
    #[prop(optional, into)]
    alignment: Signal<Option<Alignment>>,

    /// The separator type to use between breadcrumb segments.
    #[prop(optional, into)]
    separator: Signal<Option<BreadcrumbSeparator>>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        let size = size.clone();
        let alignment = alignment.clone();
        let separator = separator.clone();
        move || {
            let mut parts = vec!["breadcrumb".to_string()];
            let extra = classes.get();
            if !extra.trim().is_empty() {
                parts.push(extra);
            }
            if let Some(sz) = size.get() {
                parts.push(sz.bulma().to_string());
            }
            if let Some(align) = alignment.get() {
                parts.push(match align {
                    Alignment::Centered => "is-centered".to_string(),
                    Alignment::Right => "is-right".to_string(),
                });
            }
            if let Some(sep) = separator.get() {
                parts.push(sep.bulma().to_string());
            }
            parts.join(" ")
        }
    };

    view! {
        <nav class=move || class() aria-label="breadcrumbs">
            <ul>
                {children()}
            </ul>
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn breadcrumb_renders_base_and_children() {
        let html = view! {
            <Breadcrumb>
                <li><a href="#">"Bulma"</a></li>
                <li class="is-active"><a href="#" aria-current="page">"Breadcrumb"</a></li>
            </Breadcrumb>
        }
        .to_html();

        assert!(
            html.contains(r#"class="breadcrumb""#),
            "expected base 'breadcrumb' class; got: {}",
            html
        );
        assert!(html.contains("<ul"), "expected inner list; got: {}", html);
        assert!(
            html.contains("Bulma") && html.contains("Breadcrumb"),
            "expected children; got: {}",
            html
        );
    }

    #[test]
    fn breadcrumb_size_alignment_separator_classes() {
        let html = view! {
            <Breadcrumb
                size=leptos::prelude::Signal::derive(|| Some(BreadcrumbSize::Small))
                alignment=leptos::prelude::Signal::derive(|| Some(Alignment::Right))
                separator=leptos::prelude::Signal::derive(|| Some(BreadcrumbSeparator::Dot))
                classes="extra"
            >
                <li><a href="#">"A"</a></li>
                <li class="is-active"><a href="#" aria-current="page">"B"</a></li>
            </Breadcrumb>
        }
        .to_html();

        assert!(
            html.contains("breadcrumb extra"),
            "expected extra classes; got: {}",
            html
        );
        assert!(
            html.contains("is-small"),
            "expected size class; got: {}",
            html
        );
        assert!(
            html.contains("is-right"),
            "expected alignment class; got: {}",
            html
        );
        assert!(
            html.contains("has-dot-separator"),
            "expected separator class; got: {}",
            html
        );
    }
}
