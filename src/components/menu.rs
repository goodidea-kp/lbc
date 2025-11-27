use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, Get, IntoView, Signal, component, view,
};

use crate::util::TestAttr;

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

    /// Optional test attribute (renders as data-* attribute) on the root <aside>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (e.g., `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    /// Child content of the menu (MenuLabel, MenuList, etc.).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("menu", &classes.get())
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <aside
            class=class
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
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

    /// Optional test attribute (renders as data-* attribute) on the <ul>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("menu-list", &classes.get())
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <ul
            class=class
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
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

    /// Optional test attribute (renders as data-* attribute) on the <p>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("menu-label", &classes.get())
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <p
            class=class
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
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
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn menu_renders_test_attr_as_data_testid() {
        let html = view! {
            <Menu classes="extra" test_attr="menu-test">
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
    fn menu_no_test_attr_when_not_provided() {
        let html = view! {
            <Menu>
                <div>"X"</div>
            </Menu>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute on Menu when not provided; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn menu_list_renders_test_attr_as_data_testid() {
        let html = view! {
            <MenuList classes="extra" test_attr="menu-list-test">
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
    fn menu_label_renders_test_attr_as_data_testid() {
        let html = view! {
            <MenuLabel text="General" test_attr="menu-label-test" />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="menu-label-test""#),
            "expected data-testid attribute on MenuLabel; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn menu_accepts_custom_test_attr_key() {
        let html = view! {
            <Menu
                classes="extra"
                test_attr=TestAttr::new("data-cy", "menu-cy")
            >
                <div>"X"</div>
            </Menu>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="menu-cy""#),
            "expected custom data-cy attribute on Menu; got: {}",
            html
        );
    }
}
