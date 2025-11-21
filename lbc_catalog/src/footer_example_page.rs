/*!
Example page: Footer

Demonstrates the lbc Footer component.
*/

use lbc::prelude::{Block, Title, HeaderSize, Content, Footer};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn FooterPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Footer"</Title>

            <Footer classes="has-background-light">
                <Content classes="has-text-centered">
                    <p>
                        <strong>"LBC"</strong>
                        " by "
                        <a href="https://linkedin.com/in/konstantinpupkov">"Konstantin Pupkov"</a>
                        ". The source code is licensed "
                        <a href="https://opensource.org/licenses/mit-license.php">"MIT"</a>
                        "."
                    </p>
                </Content>
            </Footer>

            <div class="mt-4"></div>

            <Footer classes="has-background-dark has-text-white">
                <Content classes="has-text-centered">
                    <p>"Dark footer with white text"</p>
                </Content>
            </Footer>
        </Block>
    }
}
