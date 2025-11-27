/*!
Example page: Icon

Notes:
- The Icon component renders generic <i> content and typically relies on an icon font such as Font Awesome
  to display glyphs (e.g. <i class="fa fa-home"></i>).
- Make sure your index.html includes the Font Awesome stylesheet. Example (recommended CDN, v7.0.1 or newer):

  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@fortawesome/fontawesome-free@7.0.1/css/all.min.css" crossorigin="anonymous"/>

- This example performs a lightweight runtime validation: it checks for a stylesheet link that looks like
  Font Awesome (common href fragments such as "fontawesome", "fortawesome", or "all.min.css"). If not found,
  a console.warn is emitted and a small warning notification is shown on the page so you can add the link.
*/

use lbc::prelude::{Block, HeaderSize, Icon, IconAlignment, Notification, Size, Title};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};
use leptos::web_sys;

#[component]
pub fn IconPage() -> impl IntoView {
    // runtime check: detect common Font Awesome stylesheet href fragments
    let fa_present = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| {
            // look for a few common fragments used by Font Awesome CDN links
            d.query_selector("link[href*='fontawesome'],link[href*='fortawesome'],link[href*='@fortawesome'],link[href*='all.min.css']")
                .ok()
                .flatten()
        })
        .is_some();

    if !fa_present {
        // Emit a developer-facing console warning so missing dependency is obvious
        web_sys::console::warn_1(&"Font Awesome CSS not found in index.html. Icons may not render. Add: <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/@fortawesome/fontawesome-free@7.0.1/css/all.min.css\" crossorigin=\"anonymous\"/>".into());
    }

    view! {
        <Block>
            // show a visible warning in the example if the stylesheet is missing
            { if !fa_present {
                view! { <Notification classes="is-warning">"Font Awesome CSS not detected in index.html — icons may not render. Please add the stylesheet link (e.g. Font Awesome v7.0.1+)."</Notification> }
            } else {
                // return a notification with the same class and an explicit empty string child so the branch types match
                view! { <Notification classes="is-warning">""</Notification> }
            } }

            <Title size=HeaderSize::Is5>"Icon"</Title>

            <p class="mb-2">"Default icon:"</p>
            <Icon>
                <i class="fa fa-home"></i>
            </Icon>

            <div class="mt-3"></div>

            <p class="mb-2">"Small left-aligned icon with extra class:"</p>
            <Icon size=Size::Small alignment=IconAlignment::Left classes="has-text-primary">
                <i class="fa fa-user"></i>
            </Icon>

            <div class="mt-3"></div>

            <p class="mb-2">"Large right-aligned icon:"</p>
            <Icon size=Size::Large alignment=IconAlignment::Right>
                <i class="fa fa-cog"></i>
            </Icon>
        </Block>
    }
}
