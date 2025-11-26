/*!
Example page: Tag

AI Pair Programming Notes:
- Demonstrates Tag variants and props in isolation.
- Keep markup flat and readable; avoid deep nesting.
*/

use lbc::prelude::{Block, HeaderSize, Tag, TagColor, Tags, Title};
use leptos::prelude::{IntoView, component, view};

#[component]
pub fn TagPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Tag"</Title>
            <Tags>
                <Tag>"Default"</Tag>
                <Tag color=TagColor::Info>"Info"</Tag>
                <Tag color=TagColor::Warning light=true rounded=true>"Rounded Light Warning"</Tag>
            </Tags>
        </Block>
    }
}
