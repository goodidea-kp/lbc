/*!
Calendar component: a thin Leptos wrapper around the bulma-calendar JS date/time picker.

Summary
- Enhances a plain <input> with bulmaCalendar for date and time selection.
- Emits changes through a Rust callback whenever the user selects, validates, or clears.
- Requires bulmaCalendar JS and CSS to be loaded globally (available as `bulmaCalendar`).

Value format
- The emitted string follows the configured `date_format` and `time_format` patterns understood by bulmaCalendar.
- Clearing the picker emits an empty string.

Programmatic control
- To update the picker value from the outside, update the `date` signal.
- To clear the picker from the outside, set the `date` signal to a single space `" "`.

Required static assets
- CSS (add in <head>):
  https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/css/bulma-calendar.min.css
- JS (load before WASM bootstrap so `bulmaCalendar` exists):
  https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/js/bulma-calendar.min.js
*/

use leptos::html;
use leptos::prelude::Callback;
use leptos::prelude::{
    ClassAttribute, CustomAttribute, Get, GetUntracked, GlobalAttributes, IntoView, NodeRef,
    NodeRefAttribute, Signal, component, view,
};
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use leptos::web_sys::Element;

use crate::util::TestAttr;

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
    #[prop(optional, into)]
    date: Signal<String>,

    /// Callback invoked when the date/time changes; receives empty string on clear.
    update: Callback<String>,

    /// Extra classes appended after Bulma "input".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test attribute (renders as data-* attribute) on the input.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    #[prop(optional, into)] calendar_type: Signal<String>,
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
    let initial_value = date.get_untracked();
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
    let _id_for_effect = id.clone();
    let _date_sig = date.clone();
    let _calendar_type_sig = if calendar_type.get().trim().is_empty() {
        Signal::from("datetime")
    } else {
        calendar_type.clone()
    };
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
                        update.run(s);
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
                let picker_type = _calendar_type_sig.get_untracked();

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

        // Watch for changes in the date signal to update or clear the picker.
        leptos::prelude::Effect::new(move |_| {
            let current_date = _date_sig.get();
            if current_date == " " {
                clear_date(&JsValue::from(_id_for_effect.as_str()));
            } else {
                update_value(
                    &JsValue::from(_id_for_effect.as_str()),
                    &JsValue::from(current_date),
                );
            }
        });
    }

    // Detach JS state on unmount.
    #[cfg(target_arch = "wasm32")]
    leptos::prelude::on_cleanup(move || {
        detach_date_picker(&JsValue::from(_id_for_cleanup.as_str()));
    });

    // Derive specific optional attributes that our macro can render.
    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    // For SSR/tests: render a reasonable input type.
    // Existing tests expect:
    // - type="date" when time_format is empty
    // - type="datetime" when time_format is provided
    let input_type = {
        let tf = _time_format_sig.clone();
        move || {
            if tf.get().trim().is_empty() {
                "date".to_string()
            } else {
                "datetime".to_string()
            }
        }
    };

    view! {
        <input
            id=id.clone()
            class=move || class()
            type=input_type
            value=initial_value
            node_ref=input_ref
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
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
            showTodayButton: false
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
            calendarInstance.hide();
        });
    }
    if (initial_date) {
        init.get(element.id).value(initial_date);
    }
}
export function detach_date_picker(id) {
    init.delete(id);
}
export function clear_date(id) {
    if (init.has(id)) {
        init.get(id).clear();
    }
}
export function update_value(id, value) {
    if (init.has(id)) {
        init.get(id).value(value);
    }
}
"#)]
#[cfg(target_arch = "wasm32")]
#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn setup_date_picker(
        element: &Element,
        callback: &JsValue,
        initial_date: &JsValue,
        date_format: &JsValue,
        time_format: &JsValue,
        picker_type: &JsValue,
    );

    fn detach_date_picker(id: &JsValue);

    fn clear_date(id: &JsValue);

    fn update_value(id: &JsValue, value: &JsValue);
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    fn noop() -> Callback<String> {
        Callback::new(|_: String| {})
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
            <Calendar id="d".to_string() date="2025-01-01 10:00" classes="is-small" update=noop() />
        }
        .to_html();
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
                date="2025-02-03"
                date_format="yyyy-MM-dd"
                update=noop()
            />
        }
        .to_html();
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
                date="2025-02-03 12:34"
                date_format="yyyy-MM-dd"
                time_format="HH:mm"
                update=noop()
            />
        }
        .to_html();
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
        }
        .to_html();
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    fn noop() -> Callback<String> {
        Callback::new(|_: String| {})
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn calendar_renders_test_id() {
        let html = view! {
            <Calendar id="appt".to_string() update=noop() test_attr=TestAttr::test_id("calendar-test") />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="calendar-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn calendar_no_test_id_when_not_provided() {
        let html = view! {
            <Calendar id="appt".to_string() update=noop() />
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }
}
