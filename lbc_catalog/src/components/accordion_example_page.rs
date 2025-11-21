use leptos::prelude::{component, view, IntoView, ElementChild};
use lbc::prelude::{Accordions, AccordionItem, Block, Content, HeaderSize, Subtitle, Title, List};

#[component]
pub fn AccordionPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Accordion"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Bulma Accordion (click headers to toggle)"</Subtitle>

                <Accordions id="demo-accordion".to_string()>
                    <AccordionItem title="First item" open=true>
                        <p>"This is the first accordion body. Click the header to collapse."</p>
                    </AccordionItem>
                    <AccordionItem title="Second item">
                        <p>"Another section of content. Try expanding and collapsing."</p>
                    </AccordionItem>
                    <AccordionItem title="Third item">
                        <List>
                            <li>"Item A"</li>
                            <li>"Item B"</li>
                            <li>"Item C"</li>
                        </List>
                    </AccordionItem>
                </Accordions>
            </Content>
        </Block>
    }
}
