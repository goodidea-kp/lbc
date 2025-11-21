/*!
Example page: Media

AI Pair Programming Notes:
- Demonstrates Bulma Media Object with left/content/right areas.
- Kept markup simple and deterministic; no external assets.
*/

use lbc::prelude::{Button, ButtonColor, Media, MediaContent, MediaLeft, MediaRight};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn MediaPage() -> impl IntoView {
    view! {
        <div class="block">
            <h3 class="title is-5">"Media"</h3>

            <Media tag="article" classes="box">
                <MediaLeft>
                    <div class="image is-64x64">
                        <div class="has-background-grey-lighter" style="width:64px;height:64px;border-radius:4px;"></div>
                    </div>
                </MediaLeft>
                <MediaContent>
                    <div class="content">
                        <p>
                            <strong>"John Smith"</strong> <small>"@johnsmith"</small> <small>"31m"</small>
                            <br/>
                            "This is a simple media object example using LBC components."
                        </p>
                    </div>
                </MediaContent>
                <MediaRight>
                    <div class="buttons are-small">
                        <Button color=ButtonColor::Link>"Reply"</Button>
                        <Button color=ButtonColor::Light>"Share"</Button>
                    </div>
                </MediaRight>
            </Media>
        </div>
    }
}
