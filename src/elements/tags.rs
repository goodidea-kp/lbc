/*!
Leptos version of Bulma Tags container.

Bulma docs: https://bulma.io/documentation/elements/tag/#list-of-tags
*/

use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, Get, IntoView, Signal, component, view,
};

use crate::util::TestAttr;

/// A simple wrapper for a group of tags (`<div class="tags">`).
#[component]
pub fn Tags(
    /// Extra classes to apply to the root "tags" container.
    #[prop(optional, into)]
    classes: Option<Signal<String>>,

    /// Optional test attribute (renders as data-* attribute) on the root `<div>`.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (for example, `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    /// Group content (Tag components or plain elements with Bulma tag classes).
    children: Children,
) -> impl IntoView {
    let class = {
        let classes = classes.clone();
        move || {
            let mut parts: Vec<String> = vec!["tags".to_string()];
            if let Some(extra) = &classes {
                let s = extra.get();
                if !s.trim().is_empty() {
                    parts.push(s);
                }
            }
            parts.join(" ")
        }
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
    fn tags_renders_default_container() {
        let html = view! { <Tags><span>"X"</span></Tags> }.to_html();
        assert!(
            html.contains(r#"class="tags""#),
            "expected base 'tags' class, got: {}",
            html
        );
        assert!(
            html.contains(">X<"),
            "expected children to render, got: {}",
            html
        );
    }

    #[test]
    fn tags_appends_custom_classes() {
        let html =
            view! { <Tags classes="is-centered is-medium"><span>"Y"</span></Tags> }.to_html();
        assert!(
            html.contains(r#"class="tags is-centered is-medium""#),
            "expected combined classes, got: {}",
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
    fn tags_renders_test_id() {
        let html = view! {
            <Tags test_attr=TestAttr::test_id("tags-test")><span>"Content"</span></Tags>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="tags-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn tags_no_test_attr_when_not_provided() {
        let html = view! {
            <Tags><span>"Content"</span></Tags>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn tags_accepts_custom_test_attr_key() {
        let html = view! {
            <Tags test_attr=TestAttr::new("data-cy", "tags-cy")><span>"Content"</span></Tags>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="tags-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
