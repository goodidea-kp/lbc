use leptos::prelude::{component, view, Children, ClassAttribute, ElementChild, Get, IntoView, Signal};

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

    /// Child content of the menu (MenuLabel, MenuList, etc.).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("menu", &classes.get())
    };

    view! {
        <aside class=class>
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
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("menu-list", &classes.get())
    };

    view! {
        <ul class=class>
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
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || base_class("menu-label", &classes.get())
    };

    view! {
        <p class=class>
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
        assert!(html.contains(r#"class="menu""#), "expected base 'menu' class; got: {}", html);
        assert!(html.contains("X"), "expected children rendered; got: {}", html);
    }

    #[test]
    fn menu_list_renders_container() {
        let html = view! { <MenuList><li><a>"Item"</a></li></MenuList> }.to_html();
        assert!(html.contains(r#"class="menu-list""#), "expected 'menu-list' class; got: {}", html);
        assert!(html.contains("Item"), "expected list child; got: {}", html);
    }

    #[test]
    fn menu_label_renders_text() {
        let html = view! { <MenuLabel text="General" /> }.to_html();
        assert!(html.contains(r#"class="menu-label""#), "expected 'menu-label' class; got: {}", html);
        assert!(html.contains("General"), "expected label text; got: {}", html);
    }
}
