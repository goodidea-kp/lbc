use gloo_console::info;
use lbc::prelude::{
    Block, Button, Calendar, Content, Control, Field, HeaderSize, Notification, Subtitle, Title,
};
use leptos::ev::MouseEvent;
use leptos::prelude::Callback;
use leptos::prelude::{ClassAttribute, ElementChild, Get, IntoView, Set, component, signal, view};

#[component]
#[allow(non_snake_case)]
pub fn CalendarPage() -> impl IntoView {
    // Example 1: date + time
    let (selected_dt, set_selected_dt) = signal("2025-11-23 18:45".to_string());
    let on_change_dt = Callback::new(move |v: String| set_selected_dt.set(v));

    // Example 2: date only (no time)
    let (selected_d, set_selected_d) = signal("2025-12-31".to_string());
    let on_change_d = Callback::new(move |v: String| {
        info!("Selected date: {}", &v);
        set_selected_d.set(v);
    });

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
                            date=selected_dt
                            classes="is-small"
                            update=on_change_dt.clone()
                        />
                    </Control>
                </Field>
                <p class="help">"Selected (datetime): " {move || selected_dt.get()}</p>
                <Button classes="mt-4" on_click=Callback::new(move |e: MouseEvent|
                    set_selected_dt.set(" ".to_string()))
                >
                    "Clear Date/Time"
                </Button>
                <Field label="Birthday (date only)" classes="mt-4">
                    <Control>
                        <Calendar
                            id="birthday".to_string()
                            date_format="yyyy-MM-dd"
                            date=selected_d
                            classes=""
                            update=on_change_d.clone()
                        />
                    </Control>
                </Field>
                <p class="help">"Selected (date): " {move || selected_d.get()}</p>
                <Button classes="mt-4" on_click=Callback::new(move |e: MouseEvent|
                    set_selected_d.set(" ".to_string()))
                >
                    "Clear Birthday"
                </Button>

                <Notification classes="is-light mt-3">
                    "Note: bulma-calendar JS and CSS must be loaded. We've added them to index.html for the catalog."
                    <br/>
                    "To clear the calendar programmatically, set the date signal to a single space \" \"."
                </Notification>
            </Content>
        </Block>
    }
}
