/*!
Calendar component: a thin Leptos wrapper around the bulma-calendar JS date/time picker.

Summary
- Enhances a plain <input> with bulmaCalendar for date and time selection.
- Emits changes through a Rust callback whenever the user selects, validates, or clears.
- Requires bulmaCalendar JS and CSS to be loaded globally (available as `bulmaCalendar`).

Value format
- The emitted string follows the configured `date_format` and `time_format` patterns understood by bulmaCalendar.
- Clearing the picker emits an empty string.

Required static assets
- CSS (add in <head>):
  https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/css/bulma-calendar.min.css
- JS (load before WASM bootstrap so `bulmaCalendar` exists):
  https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/js/bulma-calendar.min.js
*/

use leptos::html;
use leptos::prelude::{
    ClassAttribute, Get, GetUntracked, GlobalAttributes, IntoView, NodeRef, NodeRefAttribute, Signal, component,
    view,
};
#[cfg(target_arch = "wasm32")]
use leptos::web_sys::Element;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::{JsCast, JsValue};

/// A date/time input enhanced by bulma-calendar.
///
/// Controlled outward via `update` callback. The underlying input is driven by the JS plugin.
#[component]
pub fn Calendar(
    /// Unique DOM id for the input (used to attach/detach the JS widget).
    id: String,

    /// Date format understood by bulmaCalendar. Defaults to "yyyy-MM-dd" when empty.
    #[prop(optional, into)]
    date_format: Signal<String>,

    /// Time format understood by bulmaCalendar. Defaults to "HH:mm" when empty.
    #[prop(optional, into)]
    time_format: Signal<String>,

    /// Optional initial value to seed the widget with.
    #[prop(optional)]
    date: Option<String>,

    /// Callback invoked when the date/time changes; receives empty string on clear.
    update: std::sync::Arc<dyn Fn(String) + Send + Sync>,

    /// Extra classes appended after Bulma "input".
    #[prop(optional, into)]
    classes: Signal<String>,
) -> impl IntoView {
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    // Compute Bulma "input" base class plus extras.
    let class = {
        let classes = classes.clone();
        move || {
            let extra = classes.get();
            if extra.trim().is_empty() {
                "input".to_string()
            } else {
                format!("input {}", extra)
            }
        }
    };

    // Keep simple initial value for SSR; runtime is controlled by JS widget.
    let initial_value = date.clone().unwrap_or_default();
    let _date_format_sig = date_format.clone();
    // Validate date_format strictly: allow only exact "yyyy-MM-dd" (lowercase), or empty to use default.
    {
        let df_now = _date_format_sig.get_untracked();
        let df_trim = df_now.trim();
        assert!(
            df_trim.is_empty() || df_trim == "yyyy-MM-dd",
            "Calendar date_format must be exactly 'yyyy-MM-dd' (lowercase yyyy-MM-dd). Got '{}'",
            df_now
        );
    }
    let _time_format_sig = time_format.clone();
    let _id_for_cleanup = id.clone();
    #[cfg(target_arch = "wasm32")]
    let initial_for_js = initial_value.clone();
    #[cfg(not(target_arch = "wasm32"))]
    let _ = &update;

    // Attach the JS widget once the node is mounted.
    #[cfg(target_arch = "wasm32")]
    {
        // Use an Effect to run after the initial render, when the node ref is available.
        leptos::prelude::Effect::new(move |_| {
            if let Some(input) = input_ref.get() {
                // Convert HtmlInputElement => Element for the JS bridge.
                let element: Element = input.unchecked_into();

                // Bridge from JS -> Rust, forward value through provided callback.
                let cb = {
                    let update = update.clone();
                    Closure::wrap(Box::new(move |date: JsValue| {
                        let s = date.as_string().unwrap_or_default();
                        (update)(s);
                    }) as Box<dyn FnMut(JsValue)>)
                };

                // Resolve effective formats with sane defaults.
                let df = {
                    let s = _date_format_sig.get();
                    if s.trim().is_empty() {
                        "yyyy-MM-dd".to_string()
                    } else {
                        s
                    }
                };
                let tf = {
                    let s = _time_format_sig.get();
                    if s.trim().is_empty() {
                        "HH:mm".to_string()
                    } else {
                        s
                    }
                };

                // Determine picker type based on whether time format is provided.
                let picker_type = if _time_format_sig.get_untracked().trim().is_empty() {
                    "date".to_string()
                } else {
                    "datetime".to_string()
                };

                // Seed initial value, attach plugin, and keep the closure alive.
                setup_date_picker(
                    &element,
                    cb.as_ref(),
                    &JsValue::from(initial_for_js.clone()),
                    &JsValue::from(df),
                    &JsValue::from(tf),
                    &JsValue::from(picker_type),
                );
                cb.forget();
            }
        });
    }

    // Detach JS state on unmount.
    #[cfg(target_arch = "wasm32")]
    leptos::prelude::on_cleanup(move || {
        detach_date_picker(&JsValue::from(_id_for_cleanup.as_str()));
    });

    view! {
        <input
            id=id.clone()
            class=move || class()
            type=move || {
                let s = _time_format_sig.get();
                if s.trim().is_empty() { "date".to_string() } else { "datetime".to_string() }
            }
            value=initial_value
            node_ref=input_ref
        />
    }
}

// JS bridge that attaches bulmaCalendar to the provided element and wires a change callback.
#[cfg(target_arch = "wasm32")]
#[leptos::wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
let init = new Map();
export function setup_date_picker(element, callback, initial_date, date_format, time_format, picker_type) {
    if (!init.has(element.id)) {
        let calendarInstances = bulmaCalendar.attach(element, {
            type: picker_type || (String(time_format || '').trim() ? 'datetime' : 'date'),
            color: 'info',
            lang: 'en',
            dateFormat: date_format,
            timeFormat: time_format,
        });
        init.set(element.id, calendarInstances[0]);
        let calendarInstance = calendarInstances[0];
        calendarInstance.on('select', function(datepicker) {
            callback(datepicker.data.value());
        });
        calendarInstance.on('clear', function(_datepicker) {
            callback('');
        });
        calendarInstance.on('validate', function(datepicker) {
            callback(datepicker.data.value());
        });
    }
    init.get(element.id).value(initial_date);
}
export function detach_date_picker(id) {
    init.delete(id);
}
"#)]
#[cfg(target_arch = "wasm32")]
#[allow(improper_ctypes, improper_ctypes_definitions)]
unsafe extern "C" {
    fn setup_date_picker(
        element: &Element,
        callback: &JsValue,
        initial_date: &JsValue,
        date_format: &JsValue,
        time_format: &JsValue,
        picker_type: &JsValue,
    );

    fn detach_date_picker(id: &JsValue);
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;
    use std::sync::Arc;

    fn noop() -> Arc<dyn Fn(String) + Send + Sync> {
        Arc::new(|_| {})
    }

    #[test]
    fn calendar_renders_input_with_id_and_base_class() {
        let html = view! { <Calendar id="appt".to_string() update=noop() /> }.to_html();
        assert!(
            html.contains(r#"id="appt""#),
            "expected id attribute; got: {}",
            html
        );
        assert!(
            html.contains(r#"class="input""#),
            "expected Bulma input class; got: {}",
            html
        );
    }

    #[test]
    fn calendar_initial_value_and_extra_classes() {
        let html = view! {
            <Calendar id="d".to_string() date="2025-01-01 10:00".to_string() classes="is-small" update=noop() />
        }.to_html();
        assert!(
            html.contains(r#"class="input is-small""#)
                || html.contains(r#"class="input is-small ""#),
            "expected extra classes; got: {}",
            html
        );
        assert!(
            html.contains(r#"value="2025-01-01 10:00""#),
            "expected initial value; got: {}",
            html
        );
    }

    #[test]
    fn calendar_date_only_sets_input_type_date() {
        let html = view! {
            <Calendar
                id="only-date".to_string()
                date="2025-02-03".to_string()
                date_format="yyyy-MM-dd"
                update=noop()
            />
        }.to_html();
        assert!(
            html.contains(r#"type="date""#),
            "expected input type=date when time_format is empty; got: {}",
            html
        );
    }

    #[test]
    fn calendar_datetime_sets_input_type_datetime() {
        let html = view! {
            <Calendar
                id="with-datetime".to_string()
                date="2025-02-03 12:34".to_string()
                date_format="yyyy-MM-dd"
                time_format="HH:mm"
                update=noop()
            />
        }.to_html();
        assert!(
            html.contains(r#"type="datetime""#),
            "expected input type=datetime when time_format provided; got: {}",
            html
        );
    }

    #[test]
    #[should_panic(expected = "Calendar date_format must be exactly 'yyyy-MM-dd'")]
    fn calendar_rejects_invalid_date_format_uppercase() {
        // Using uppercase tokens should be rejected
        let _ = view! {
            <Calendar
                id="bad-format".to_string()
                date_format="YYYY-MM-DD"
                update=noop()
            />
        }.to_html();
    }
}
