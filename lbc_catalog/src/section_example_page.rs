/*!
Example page: Section

AI Pair Programming Notes:
- Demonstrates Bulma Section layout with default and medium size variants.
*/

use lbc::prelude::{Container, Section, SectionSize};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn SectionPage() -> impl IntoView {
    view! {
        <div class="block">
            <h3 class="title is-5">"Section"</h3>

            <Section>
                <Container>
                    <p class="title is-6">"Default section"</p>
                    <p>"This is a standard Bulma section with container."</p>
                </Container>
            </Section>

            <Section size=SectionSize::Medium>
                <Container>
                    <p class="title is-6">"Medium section"</p>
                    <p>"This section has the is-medium size modifier."</p>
                </Container>
            </Section>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lbc::prelude::{Section, SectionSize};
    use leptos::prelude::RenderHtml;

    #[test]
    fn section_medium_has_class() {
        let html =
            view! { <Section size=SectionSize::Medium><div class="container"></div></Section> }
                .to_html();
        assert!(
            html.contains(r#"class="section is-medium""#),
            "expected 'section is-medium' class, got: {}",
            html
        );
    }
}
