/*!
Leptos version of Bulma Hero component.

- Hero: imposing hero banner to showcase something
- HeroSize: Medium, Large, Fullheight, FullheightWithNavbar

Follows existing crate patterns:
- optional props via #[prop(optional)]
- classes as Option<Signal<String>>
*/

use leptos::prelude::{
    AnyView, ClassAttribute, CustomAttribute, ElementChild, Get, IntoAny, IntoView, Signal,
    component, view,
};

/// The 4 sizes available for heroes.
///
/// https://bulma.io/documentation/layout/hero/#sizes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeroSize {
    Medium,
    Large,
    Fullheight,
    FullheightWithNavbar,
}

impl HeroSize {
    pub fn bulma(self) -> &'static str {
        match self {
            HeroSize::Medium => "is-medium",
            HeroSize::Large => "is-large",
            HeroSize::Fullheight => "is-fullheight",
            HeroSize::FullheightWithNavbar => "is-fullheight-with-navbar",
        }
    }
}

/// An imposing hero banner to showcase something.
///
/// https://bulma.io/documentation/layout/hero/
#[component]
pub fn Hero<B, BIV>(
    /// The contents of the hero-body section.
    body: B,
    /// Optional classes to add to the hero-body container.
    #[prop(optional, into)]
    body_classes: Option<Signal<String>>,
    /// The contents of the hero-head section.
    #[prop(optional, into)]
    head: Option<AnyView>,
    /// Optional classes to add to the hero-head container.
    #[prop(optional, into)]
    head_classes: Option<Signal<String>>,
    /// The contents of the hero-foot section.
    #[prop(optional, into)]
    foot: Option<AnyView>,
    /// Optional classes to add to the hero-foot container.
    #[prop(optional, into)]
    foot_classes: Option<Signal<String>>,
    /// Extra classes for the hero container.
    #[prop(optional, into)]
    classes: Option<Signal<String>>,
    /// The size for this hero.
    #[prop(optional)]
    size: Option<HeroSize>,
    /// Generate a subtle gradient for the hero.
    #[prop(optional)]
    bold: bool,
    /// If you are using a fixed navbar, you can use the `fixed_nav=true` modifier
    /// for it to occupy the viewport height minus the navbar height.
    ///
    /// https://bulma.io/documentation/layout/hero/#fullheight-with-navbar
    #[prop(optional)]
    fixed_nav: bool,

    /// Optional test identifier (renders as data-testid attribute) on the root <section>.
    #[prop(optional, into)]
    test_id: Option<String>,
) -> impl IntoView
where
    B: Fn() -> BIV + 'static,
    BIV: IntoView,
{
    // Build the main hero class
    let class = move || {
        let mut class_parts: Vec<&str> = vec!["hero"];

        if let Some(size) = size {
            class_parts.push(size.bulma());
        }

        if bold {
            class_parts.push("is-bold");
        }

        if fixed_nav {
            class_parts.push("is-fullheight-with-navbar");
        }

        if let Some(class_signal) = &classes {
            let extra_classes = class_signal.get();
            if !extra_classes.is_empty() {
                return format!("{} {}", class_parts.join(" "), extra_classes);
            }
        }
        class_parts.join(" ")
    };

    // Build hero-head class
    let head_class = move || {
        let mut class_parts = vec!["hero-head"];
        if let Some(hc) = &head_classes {
            let extra = hc.get();
            if !extra.is_empty() {
                return format!("{} {}", class_parts.join(" "), extra);
            }
        }
        class_parts.join(" ")
    };

    // Build hero-body class
    let body_class = move || {
        let mut class_parts = vec!["hero-body"];
        if let Some(bc) = &body_classes {
            let extra = bc.get();
            if !extra.is_empty() {
                return format!("{} {}", class_parts.join(" "), extra);
            }
        }
        class_parts.join(" ")
    };

    // Build hero-foot class
    let foot_class = move || {
        let mut class_parts = vec!["hero-foot"];
        if let Some(fc) = &foot_classes {
            let extra = fc.get();
            if !extra.is_empty() {
                return format!("{} {}", class_parts.join(" "), extra);
            }
        }
        class_parts.join(" ")
    };

    // Normalize optional slots into AnyView
    let head_view: AnyView = head.unwrap_or_else(|| view! { <div></div> }.into_any());
    let foot_view: AnyView = foot.unwrap_or_else(|| view! { <div></div> }.into_any());

    view! {
        <section class=class data-testid=test_id>
            <div class=head_class>{head_view}</div>
            <div class=body_class>{body()}</div>
            <div class=foot_class>{foot_view}</div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::{IntoAny, RenderHtml};

    #[test]
    fn hero_renders_basic() {
        let html = view! {
            <Hero
                body=|| view! { <p>"Body content"</p> }
            />
        }
        .to_html();
        assert!(
            html.contains(r#"class="hero""#),
            "expected base 'hero' class, got: {}",
            html
        );
        assert!(
            html.contains(r#"class="hero-body""#),
            "expected 'hero-body' class, got: {}",
            html
        );
        assert!(
            html.contains("Body content"),
            "expected body content, got: {}",
            html
        );
    }

    #[test]
    fn hero_with_size_and_bold() {
        let html = view! {
            <Hero
                body=|| view! { <p>"Large Hero"</p> }
                size=HeroSize::Large
                bold=true
                head={view! { <div></div> }.into_any()}
                foot={view! { <div></div> }.into_any()}
            />
        }
        .to_html();
        assert!(
            html.contains("hero is-large is-bold"),
            "expected size and bold classes, got: {}",
            html
        );
    }

    #[test]
    fn hero_with_head_and_foot() {
        let html = view! {
            <Hero
                body=|| view! { <p>"Body"</p> }
                head={view! { <nav>"Header"</nav> }.into_any()}
                foot={view! { <div>"Footer"</div> }.into_any()}
            />
        }
        .to_html();
        assert!(
            html.contains(r#"class="hero-head""#),
            "expected hero-head, got: {}",
            html
        );
        assert!(
            html.contains(r#"class="hero-foot""#),
            "expected hero-foot, got: {}",
            html
        );
        assert!(
            html.contains("Header"),
            "expected header content, got: {}",
            html
        );
        assert!(
            html.contains("Footer"),
            "expected footer content, got: {}",
            html
        );
    }

    #[test]
    fn hero_with_custom_classes() {
        let html = view! {
            <Hero
                body=|| view! { <p>"X"</p> }
                classes="is-primary"
                body_classes="has-text-centered"
                head={view! { <div>"H"</div> }.into_any()}
                head_classes="custom-head"
                foot={view! { <div></div> }.into_any()}
            />
        }
        .to_html();
        assert!(
            html.contains("hero is-primary"),
            "expected hero with custom class, got: {}",
            html
        );
        assert!(
            html.contains("hero-body has-text-centered"),
            "expected body with custom class, got: {}",
            html
        );
        assert!(
            html.contains("hero-head custom-head"),
            "expected head with custom class, got: {}",
            html
        );
    }

    #[test]
    fn hero_fullheight_with_navbar() {
        let html = view! {
            <Hero
                body=|| view! { <p>"X"</p> }
                size=HeroSize::FullheightWithNavbar
                head={view! { <div></div> }.into_any()}
                foot={view! { <div></div> }.into_any()}
            />
        }
        .to_html();
        assert!(
            html.contains("is-fullheight-with-navbar"),
            "expected fullheight-with-navbar class, got: {}",
            html
        );
    }

    #[test]
    fn hero_fixed_nav_flag() {
        let html = view! {
            <Hero
                body=|| view! { <p>"X"</p> }
                fixed_nav=true
                head={view! { <div></div> }.into_any()}
                foot={view! { <div></div> }.into_any()}
            />
        }
        .to_html();
        assert!(
            html.contains("is-fullheight-with-navbar"),
            "expected fixed_nav to add fullheight-with-navbar class, got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use leptos::prelude::*;
    use leptos::prelude::IntoAny;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn body() -> impl Fn() -> AnyView {
        || view! { <p>"Body"</p> }.into_any()
    }

    #[wasm_bindgen_test]
    fn hero_renders_test_id() {
        let html = view! {
            <Hero
                body=body()
                size=HeroSize::Medium
                bold=true
                fixed_nav=true
                classes="is-primary"
                test_id="hero-test"
                head={view! { <div>"H"</div> }.into_any()}
                foot={view! { <div>"F"</div> }.into_any()}
            />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="hero-test""#),
            "expected data-testid attribute on Hero; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn hero_no_test_id_when_not_provided() {
        let html = view! {
            <Hero
                body=body()
                size=HeroSize::Medium
                bold=true
                fixed_nav=true
                classes="is-primary"
                head={view! { <div>"H"</div> }.into_any()}
                foot={view! { <div>"F"</div> }.into_any()}
            />
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on Hero when not provided; got: {}",
            html
        );
    }
}
