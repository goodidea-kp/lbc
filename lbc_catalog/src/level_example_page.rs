/*!
Example page: Level

AI Pair Programming Notes:
- Demonstrates Bulma Level horizontal alignment layout using lbc Level components.
*/

use lbc::prelude::{Button, ButtonColor, Level, LevelItem, LevelLeft, LevelRight};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn LevelPage() -> impl IntoView {
    view! {
        <div class="block">
            <h3 class="title is-5">"Level"</h3>

            <Level>
                <LevelLeft>
                    <LevelItem>
                        <p class="subtitle is-6">"Left"</p>
                    </LevelItem>
                    <LevelItem>
                        <p>"Docs"</p>
                    </LevelItem>
                </LevelLeft>
                <LevelRight>
                    <LevelItem>
                        <Button color=ButtonColor::Link>"Action"</Button>
                    </LevelItem>
                    <LevelItem>
                        <Button color=ButtonColor::Light>"Cancel"</Button>
                    </LevelItem>
                </LevelRight>
            </Level>

            <div class="block mt-5">
                <h4 class="title is-6">"Centered Level"</h4>
                <Level tag="div">
                    <LevelItem tag="p">
                        <div>
                            <p class="heading">"Tweets"</p>
                            <p class="title">"3,456"</p>
                        </div>
                    </LevelItem>
                    <LevelItem tag="p">
                        <div>
                            <p class="heading">"Following"</p>
                            <p class="title">"123"</p>
                        </div>
                    </LevelItem>
                    <LevelItem tag="p">
                        <div>
                            <p class="heading">"Followers"</p>
                            <p class="title">"456K"</p>
                        </div>
                    </LevelItem>
                    <LevelItem tag="p">
                        <div>
                            <p class="heading">"Likes"</p>
                            <p class="title">"789"</p>
                        </div>
                    </LevelItem>
                </Level>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lbc::prelude::{Level, LevelItem, LevelLeft};
    use leptos::prelude::RenderHtml;

    #[test]
    fn level_has_items() {
        let html = view! {
            <Level>
                <LevelLeft>
                    <LevelItem>"L"</LevelItem>
                </LevelLeft>
            </Level>
        }
        .to_html();
        assert!(html.contains(r#"class="level""#), "expected 'level' class");
        assert!(html.contains("level-item"), "expected level-item");
    }
}
