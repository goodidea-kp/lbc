use leptos::prelude::{component, create_signal, view, ClassAttribute, Get, IntoView, Set, ElementChild};
use lbc::prelude::{Block, Content, Title, Subtitle, HeaderSize, Field, Control, Notification, Calendar};
use std::sync::Arc;

#[component]
pub fn CalendarPage() -> impl IntoView {
    // Example 1: date + time
    let (selected_dt, set_selected_dt) = create_signal(String::new());
    let on_change_dt = Arc::new(move |v: String| set_selected_dt.set(v));

    // Example 2: date only (no time)
    let (selected_d, set_selected_d) = create_signal(String::new());
    let on_change_d = Arc::new(move |v: String| set_selected_d.set(v));

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Calendar"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"bulma‑calendar integration"</Subtitle>

                <Field label="Appointment (date + time)">
                    <Control>
                        <Calendar
                            id="appointment".to_string()
                            date_format="yyyy-MM-dd"
                            time_format="HH:mm"
                            date="2025-11-23 18:45".to_string()
                            classes="is-small"
                            update=on_change_dt.clone()
                        />
                    </Control>
                </Field>
                <p class="help">"Selected (datetime): " {move || selected_dt.get()}</p>

                <Field label="Birthday (date only)" classes="mt-4">
                    <Control>
                        <Calendar
                            id="birthday".to_string()
                            date_format="yyyy-MM-dd"
                            date="2025-12-31".to_string()
                            classes=""
                            update=on_change_d.clone()
                        />
                    </Control>
                </Field>
                <p class="help">"Selected (date): " {move || selected_d.get()}</p>

                <Notification classes="is-light mt-3">
                    "Note: bulma-calendar JS and CSS must be loaded. We've added them to index.html for the catalog."
                </Notification>
            </Content>
        </Block>
    }
}
