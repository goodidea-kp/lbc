use leptos::prelude::{
    Children, ClassAttribute, ElementChild, Get, IntoView, Signal, component, view,
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

    /// Card body content (header, image, content, footer, etc.).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("card", &classes.get())
    };

    view! {
        <div class=class>
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

    /// Children rendered in the header (e.g., title, icons).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("card-header", &classes.get())
    };

    view! {
        <header class=class>
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

    /// Typically contains a Bulma "image" container.
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("card-image", &classes.get())
    };

    view! {
        <div class=class>
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

    /// Body content of the card.
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("card-content", &classes.get())
    };

    view! {
        <div class=class>
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

    /// Footer items (commonly multiple <a class="card-footer-item">).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("card-footer", &classes.get())
    };

    view! {
        <footer class=class>
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
