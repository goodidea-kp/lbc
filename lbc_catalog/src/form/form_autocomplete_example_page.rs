use leptos::prelude::{component, view, IntoView, ClassAttribute, ElementChild, Get, Set, Update, create_signal};
use lbc::prelude::{AutoComplete, Block, Content, HeaderSize, Subtitle, Title};
use std::sync::Arc;

/// Example page showcasing the AutoComplete component powered by Bulma TagsInput.
#[component]
pub fn FormAutoCompletePage() -> impl IntoView {
    // Track selected tags from callbacks.
    let (selected, set_selected) = create_signal::<Vec<String>>(Vec::new());

    // Merge new value into the set
    let on_add = {
        let set_selected = set_selected.clone();
        Arc::new(move |v: String| {
            set_selected.update(|list| {
                if !list.iter().any(|item| item == &v) {
                    list.push(v.clone());
                }
            });
        })
    };

    // Remove value from the set
    let on_remove = {
        let set_selected = set_selected.clone();
        Arc::new(move |v: String| {
            set_selected.update(|list| {
                list.retain(|item| item != &v);
            });
        })
    };

    let items = vec![
        "Rust".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "JavaScript".to_string(),
        "TypeScript".to_string(),
    ];

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: AutoComplete (Bulma TagsInput)"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Static list (select + tags)"</Subtitle>
                <AutoComplete
                    id="tags-static".to_string()
                    items=items.clone()
                    placeholder="Choose Tags"
                    _on_update=on_add.clone()
                    _on_remove=on_remove.clone()
                />

                <p class="help mt-3">
                    "Selected tags: "
                    {move || {
                        let s = selected.get();
                        if s.is_empty() { "(none)".to_string() } else { s.join(", ") }
                    }}
                </p>
            </Content>
        </Block>
    }
}
