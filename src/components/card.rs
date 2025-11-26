use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, Get, IntoView, Signal, component, view,
};

fn base_class(root: &str, extra: &str) -> String {
    if extra.trim().is_empty() {
        root.to_string()
    } else {
        format!("{root} {extra}")
    }
}

/// An all-around flexible and composable component; this is the card container.
/// https://bulma.io/documentation/components/card/
#[component]
pub fn Card(
    /// Extra classes to apply to the Bulma "card" container.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

    /// Card body content (header, image, content, footer, etc.).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("card", &classes.get())
    };

    view! {
        <div class=class data-testid=test_id>
            {children()}
        </div>
    }
}

/// A container for card header content; rendered as a horizontal bar with a shadow.
/// https://bulma.io/documentation/components/card/
#[component]
pub fn CardHeader(
    /// Extra classes for the "card-header".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

    /// Children rendered in the header (e.g., title, icons).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("card-header", &classes.get())
    };

    view! {
        <header class=class data-testid=test_id>
            {children()}
        </header>
    }
}

/// A fullwidth container for a responsive image.
/// https://bulma.io/documentation/components/card/
#[component]
pub fn CardImage(
    /// Extra classes for the "card-image".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

    /// Typically contains a Bulma "image" container.
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("card-image", &classes.get())
    };

    view! {
        <div class=class data-testid=test_id>
            {children()}
        </div>
    }
}

/// A container for any other content as the body of the card.
/// https://bulma.io/documentation/components/card/
#[component]
pub fn CardContent(
    /// Extra classes for the "card-content".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

    /// Body content of the card.
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("card-content", &classes.get())
    };

    view! {
        <div class=class data-testid=test_id>
            {children()}
        </div>
    }
}

/// A container for card footer content; rendered as a horizontal list of controls.
/// https://bulma.io/documentation/components/card/
#[component]
pub fn CardFooter(
    /// Extra classes for the "card-footer".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

    /// Footer items (commonly multiple <a class="card-footer-item">).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("card-footer", &classes.get())
    };

    view! {
        <footer class=class data-testid=test_id>
            {children()}
        </footer>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn card_renders_container_and_children() {
        let html = view! {
            <Card>
                <div>"X"</div>
            </Card>
        }
        .to_html();

        assert!(
            html.contains(r#"class="card""#),
            "expected base 'card' class; got: {}",
            html
        );
        assert!(
            html.contains(">X<"),
            "expected child content; got: {}",
            html
        );
    }

    #[test]
    fn card_sections_have_proper_classes() {
        let html = view! {
            <Card>
                <CardHeader classes="has-background-light"><p>"Header"</p></CardHeader>
                <CardImage><figure class="image is-4by3"><img src="#" alt=""/></figure></CardImage>
                <CardContent><p>"Body"</p></CardContent>
                <CardFooter>
                    <a class="card-footer-item">"One"</a>
                    <a class="card-footer-item">"Two"</a>
                </CardFooter>
            </Card>
        }
        .to_html();

        assert!(
            html.contains(r#"class="card-header has-background-light""#)
                || html.contains("card-header has-background-light "),
            "expected header classes; got: {}",
            html
        );
        assert!(
            html.contains(r#"class="card-image""#),
            "expected card-image class; got: {}",
            html
        );
        assert!(
            html.contains(r#"class="card-content""#),
            "expected card-content class; got: {}",
            html
        );
        assert!(
            html.contains(r#"class="card-footer""#),
            "expected card-footer class; got: {}",
            html
        );
        assert!(
            html.contains("card-footer-item"),
            "expected footer items; got: {}",
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
    fn card_renders_test_id() {
        let html = view! {
            <Card classes="extra" test_id="card-test">
                <div>"X"</div>
            </Card>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="card-test""#),
            "expected data-testid attribute on Card; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn card_no_test_id_when_not_provided() {
        let html = view! {
            <Card>
                <div>"X"</div>
            </Card>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid on Card when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn card_header_renders_test_id() {
        let html = view! {
            <CardHeader classes="extra" test_id="card-header-test">
                <p>"Header"</p>
            </CardHeader>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="card-header-test""#),
            "expected data-testid on CardHeader; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn card_image_renders_test_id() {
        let html = view! {
            <CardImage test_id="card-image-test">
                <figure class="image is-4by3"><img src="#" alt=""/></figure>
            </CardImage>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="card-image-test""#),
            "expected data-testid on CardImage; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn card_content_renders_test_id() {
        let html = view! {
            <CardContent test_id="card-content-test">
                <p>"Body"</p>
            </CardContent>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="card-content-test""#),
            "expected data-testid on CardContent; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn card_footer_renders_test_id() {
        let html = view! {
            <CardFooter test_id="card-footer-test">
                <a class="card-footer-item">"One"</a>
            </CardFooter>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="card-footer-test""#),
            "expected data-testid on CardFooter; got: {}",
            html
        );
    }
}
