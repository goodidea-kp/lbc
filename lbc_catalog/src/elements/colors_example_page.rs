/*!
Example page: Colors

AI Pair Programming Notes:
- Showcases color variants for Button and Tag.
- Tests use SSR RenderHtml to validate Bulma class application.
*/

use lbc::prelude::{
    Block, Button, ButtonColor, Buttons, Content, HeaderSize, Subtitle, Tag, TagColor, Tags, Title,
};
use leptos::prelude::{IntoView, component, view};

#[component]
pub fn ColorsPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Colors"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Button Colors"</Subtitle>
                <Buttons>
                    <Button color=ButtonColor::Primary>"Primary"</Button>
                    <Button color=ButtonColor::Link>"Link"</Button>
                    <Button color=ButtonColor::Info>"Info"</Button>
                    <Button color=ButtonColor::Success>"Success"</Button>
                    <Button color=ButtonColor::Warning>"Warning"</Button>
                    <Button color=ButtonColor::Danger>"Danger"</Button>
                    <Button color=ButtonColor::Dark>"Dark"</Button>
                    <Button color=ButtonColor::Light>"Light"</Button>
                    <Button color=ButtonColor::Black>"Black"</Button>
                </Buttons>
            </Content>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Tag Colors"</Subtitle>
                <Tags>
                    <Tag color=TagColor::Primary>"Primary"</Tag>
                    <Tag color=TagColor::Link>"Link"</Tag>
                    <Tag color=TagColor::Info>"Info"</Tag>
                    <Tag color=TagColor::Success>"Success"</Tag>
                    <Tag color=TagColor::Warning>"Warning"</Tag>
                    <Tag color=TagColor::Danger>"Danger"</Tag>
                    <Tag color=TagColor::Dark>"Dark"</Tag>
                    <Tag color=TagColor::Light>"Light"</Tag>
                    <Tag color=TagColor::Black>"Black"</Tag>
                </Tags>
            </Content>
        </Block>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn colors_page_button_primary_has_class() {
        let html = view! { <Button color=ButtonColor::Primary>"X"</Button> }.to_html();
        assert!(
            html.contains(r#"class="button is-primary""#),
            "expected Bulma primary class in: {}",
            html
        );
    }

    #[test]
    fn colors_page_tag_warning_has_class() {
        let html = view! { <Tag color=TagColor::Warning>"X"</Tag> }.to_html();
        assert!(
            html.contains(r#"class="tag is-warning""#),
            "expected Bulma warning class in: {}",
            html
        );
    }
}
