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

/// A simple menu, for any type of vertical navigation.
/// https://bulma.io/documentation/components/menu/
#[component]
pub fn Menu(
    /// Extra classes to apply to the Bulma "menu" container.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,

    /// Child content of the menu (MenuLabel, MenuList, etc.).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("menu", &classes.get())
    };

    view! {
        <aside class=class data-testid=test_id>
            {children()}
        </aside>
    }
}

/// A container for menu list `li` elements.
/// https://bulma.io/documentation/components/menu/
#[component]
pub fn MenuList(
    /// The child `li` elements of this list.
    children: Children,

    /// Extra classes for the "menu-list" container.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("menu-list", &classes.get())
    };

    view! {
        <ul class=class data-testid=test_id>
            {children()}
        </ul>
    }
}

/// A label for a section of the menu.
/// https://bulma.io/documentation/components/menu/
#[component]
pub fn MenuLabel(
    /// Extra classes for the "menu-label" element.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// The text of the label.
    #[prop(optional, into)]
    text: Signal<String>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("menu-label", &classes.get())
    };

    view! {
        <p class=class data-testid=test_id>
            {text.get()}
        </p>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn menu_renders_base_class_and_children() {
        let html = view! { <Menu><div>"X"</div></Menu> }.to_html();
        assert!(
            html.contains(r#"class="menu""#),
            "expected base 'menu' class; got: {}",
            html
        );
        assert!(
            html.contains("X"),
            "expected children rendered; got: {}",
            html
        );
    }

    #[test]
    fn menu_list_renders_container() {
        let html = view! { <MenuList><li><a>"Item"</a></li></MenuList> }.to_html();
        assert!(
            html.contains(r#"class="menu-list""#),
            "expected 'menu-list' class; got: {}",
            html
        );
        assert!(html.contains("Item"), "expected list child; got: {}", html);
    }

    #[test]
    fn menu_label_renders_text() {
        let html = view! { <MenuLabel text="General" /> }.to_html();
        assert!(
            html.contains(r#"class="menu-label""#),
            "expected 'menu-label' class; got: {}",
            html
        );
        assert!(
            html.contains("General"),
            "expected label text; got: {}",
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
    fn menu_renders_test_id() {
        let html = view! {
            <Menu classes="extra" test_id="menu-test">
                <div>"X"</div>
            </Menu>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="menu-test""#),
            "expected data-testid attribute on Menu; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn menu_no_test_id_when_not_provided() {
        let html = view! {
            <Menu>
                <div>"X"</div>
            </Menu>
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute on Menu when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn menu_list_renders_test_id() {
        let html = view! {
            <MenuList classes="extra" test_id="menu-list-test">
                <li><a>"Item"</a></li>
            </MenuList>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="menu-list-test""#),
            "expected data-testid attribute on MenuList; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn menu_label_renders_test_id() {
        let html = view! {
            <MenuLabel text="General" test_id="menu-label-test" />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="menu-label-test""#),
            "expected data-testid attribute on MenuLabel; got: {}",
            html
        );
    }
}
