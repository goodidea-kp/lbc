/*!
Example page: Title

AI Pair Programming Notes:
- Single-responsibility component focused on demonstrating the Title and Subtitle API.
- Keep imports minimal and explicit to reduce cognitive load.
- Keep examples deterministic and small; avoid hidden state outside this module.
*/

use lbc::prelude::{Block, Content, HeaderSize, Subtitle, Title};
use leptos::prelude::{ElementChild, IntoView, component, view};

#[component]
pub fn TitlePage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Title & Subtitle"</Title>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Basic Titles"</Subtitle>
                <Title>"Title"</Title>
                <Subtitle>"Subtitle"</Subtitle>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Sizes"</Subtitle>
                <Title size=HeaderSize::Is1>"Title 1"</Title>
                <Title size=HeaderSize::Is2>"Title 2"</Title>
                <Title size=HeaderSize::Is3>"Title 3"</Title>
                <Title size=HeaderSize::Is4>"Title 4"</Title>
                <Title size=HeaderSize::Is5>"Title 5"</Title>
                <Title size=HeaderSize::Is6>"Title 6"</Title>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Subtitle Sizes"</Subtitle>
                <Subtitle size=HeaderSize::Is1>"Subtitle 1"</Subtitle>
                <Subtitle size=HeaderSize::Is2>"Subtitle 2"</Subtitle>
                <Subtitle size=HeaderSize::Is3>"Subtitle 3"</Subtitle>
                <Subtitle size=HeaderSize::Is4>"Subtitle 4"</Subtitle>
                <Subtitle size=HeaderSize::Is5>"Subtitle 5"</Subtitle>
                <Subtitle size=HeaderSize::Is6>"Subtitle 6"</Subtitle>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Combining Titles and Subtitles"</Subtitle>
                <Title size=HeaderSize::Is1>"Hello World"</Title>
                <Subtitle size=HeaderSize::Is3>"A modern framework for building web applications"</Subtitle>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Spaced Titles"</Subtitle>
                <Title size=HeaderSize::Is2 is_spaced=true>"This title has spacing"</Title>
                <Subtitle size=HeaderSize::Is4>"This subtitle comes after a spaced title"</Subtitle>
                <Content>
                    <p>"When you use is_spaced on a title, it maintains normal spacing between the title and subtitle."</p>
                </Content>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Custom Tags"</Subtitle>
                <Title tag="h1" size=HeaderSize::Is2>"Title with h1 tag"</Title>
                <Subtitle tag="h2" size=HeaderSize::Is4>"Subtitle with h2 tag"</Subtitle>
                <Title tag="p" size=HeaderSize::Is5>"Title with p tag"</Title>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Text Colors"</Subtitle>
                <Title size=HeaderSize::Is3 classes="has-text-primary">"Primary Title"</Title>
                <Title size=HeaderSize::Is3 classes="has-text-info">"Info Title"</Title>
                <Title size=HeaderSize::Is3 classes="has-text-success">"Success Title"</Title>
                <Title size=HeaderSize::Is3 classes="has-text-warning">"Warning Title"</Title>
                <Title size=HeaderSize::Is3 classes="has-text-danger">"Danger Title"</Title>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Text Alignment"</Subtitle>
                <Title size=HeaderSize::Is4 classes="has-text-centered">"Centered Title"</Title>
                <Title size=HeaderSize::Is4 classes="has-text-right">"Right-aligned Title"</Title>
                <Title size=HeaderSize::Is4 classes="has-text-left">"Left-aligned Title"</Title>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Combined Styling"</Subtitle>
                <Title
                    tag="h1"
                    size=HeaderSize::Is2
                    is_spaced=true
                    classes="has-text-centered has-text-primary"
                >
                    "Featured Article"
                </Title>
                <Subtitle
                    tag="h2"
                    size=HeaderSize::Is4
                    classes="has-text-centered has-text-grey"
                >
                    "A comprehensive guide to modern web development"
                </Subtitle>
            </Block>
        </Block>
    }
}
