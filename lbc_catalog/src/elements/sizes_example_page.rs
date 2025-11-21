/*!
Example page: Sizes

AI Pair Programming Notes:
- Demonstrates Size usage across Button and Tag components.
- Tests verify Size -> Bulma class mapping without requiring SSR.
*/

use lbc::prelude::{Block, Title, Subtitle, HeaderSize, Content, Buttons, Button, ButtonColor, Size, Tag, TagColor, Tags};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn SizesPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Sizes"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Buttons"</Subtitle>
                <Buttons>
                    <Button size=Size::Small color=ButtonColor::Primary>"Small"</Button>
                    <Button color=ButtonColor::Info>"Normal (default)"</Button>
                    <Button size=Size::Medium color=ButtonColor::Success>"Medium"</Button>
                    <Button size=Size::Large color=ButtonColor::Warning>"Large"</Button>
                </Buttons>
            </Content>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Tags"</Subtitle>
                <Tags>
                    <Tag size=Size::Small color=TagColor::Primary>"Small"</Tag>
                    <Tag color=TagColor::Dark>"Normal (default)"</Tag>
                    <Tag size=Size::Medium color=TagColor::Info>"Medium"</Tag>
                    <Tag size=Size::Large color=TagColor::Warning>"Large"</Tag>
                </Tags>
            </Content>
        </Block>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lbc::prelude::Size;

    #[test]
    fn size_bulma_mapping_is_correct() {
        assert_eq!(Size::Small.bulma(), "is-small");
        assert_eq!(Size::Normal.bulma(), "");
        assert_eq!(Size::Medium.bulma(), "is-medium");
        assert_eq!(Size::Large.bulma(), "is-large");
    }
}
