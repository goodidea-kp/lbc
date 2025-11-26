/*!
Calendar component: a thin Leptos wrapper around the bulma-calendar JS date/time picker.

Summary
- Enhances a plain <input> with bulmaCalendar for date and time selection.
- Emits changes through a Rust callback whenever the user selects, validates, clears, or clicks Today/Cancel.
- Requires bulmaCalendar JS and CSS to be loaded globally (available as `bulmaCalendar`).

Value format
- The emitted string follows the configured `date_format` and `time_format` patterns understood by bulmaCalendar.
- Clearing or canceling the picker emits an empty string.

Required static assets
- CSS (add in <head>):
  https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/css/bulma-calendar.min.css
- JS (load before WASM bootstrap so `bulmaCalendar` exists):
  https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/js/bulma-calendar.min.js
*/

use leptos::html;
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
    #[prop(optional)]
    date: Option<String>,

    /// Callback invoked when the date/time changes; receives empty string on clear/cancel.
    update: std::sync::Arc<dyn Fn(String) + Send + Sync>,

    /// Extra classes appended after Bulma "input".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Optional test attribute (renders as data-* attribute) on the input.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
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

                // Bridge from JS -> Rust, forward value through provided callback (change events).
                let cb_change = {
                    let update = update.clone();
                    Closure::wrap(Box::new(move |date: JsValue| {
                        let s = date.as_string().unwrap_or_default();
                        leptos::logging::log!("[Calendar] Rust cb_change received: {s}");
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

                leptos::logging::log!(
                    "[Calendar] Rust calling setup_date_picker with picker_type={picker_type}"
                );

                // Seed initial value, attach plugin, and keep the closure alive.
                setup_date_picker(
                    &element,
                    cb_change.as_ref(),
                    &JsValue::from(initial_for_js.clone()),
                    &JsValue::from(df),
                    &JsValue::from(tf),
                    &JsValue::from(picker_type),
                );
                // Prevent GC of the closure by leaking it intentionally for widget lifetime.
                cb_change.forget();
            } else {
                leptos::logging::log!("[Calendar] Rust Effect: input_ref.get() returned None");
            }
        });
    }

    // Detach JS state on unmount.
    #[cfg(target_arch = "wasm32")]
    leptos::prelude::on_cleanup(move || {
        leptos::logging::log!(
            "[Calendar] Rust on_cleanup detaching date picker for id={}",
            _id_for_cleanup
        );
        detach_date_picker(&JsValue::from(_id_for_cleanup.as_str()));
    });

    // Derive specific optional attributes that our macro can render.
    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

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
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        />
    }
}

// JS bridge that attaches bulmaCalendar to the provided element and wires a change callback.
//
// This implementation listens to several events, including:
// - "select" / "validate" for normal date changes
// - "clear" / "onCancelClickDateTimePicker" for clearing the value
// - "onTodayClickDateTimePicker" / "today" for clicking the Today button
//
// All of these events call back into Rust via `on_change`.
#[cfg(target_arch = "wasm32")]
#[leptos::wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
let init = new Map();

/**
 * Attach bulmaCalendar to the given element and wire up Rust callbacks.
 *
 * @param {Element} element - the input element
 * @param {Function} on_change - Rust callback for general value changes (select/validate/clear/cancel/today)
 * @param {string} initial_date - initial value
 * @param {string} date_format - date format string
 * @param {string} time_format - time format string
 * @param {string} picker_type - "date" or "datetime"
 */
export function setup_date_picker(element, on_change, initial_date, date_format, time_format, picker_type) {
    console.log('[Calendar] setup_date_picker: entering', {
        element,
        elementId: element && element.id,
        hasOnChange: typeof on_change === 'function',
        initial_date,
        date_format,
        time_format,
        picker_type,
        bulmaCalendarType: typeof bulmaCalendar
    });

    if (!element || !element.id) {
        console.warn('[Calendar] setup_date_picker: element or element.id missing');
    }

    if (!init.has(element.id)) {
        if (typeof bulmaCalendar === 'undefined') {
            console.error('[Calendar] bulmaCalendar is undefined at setup_date_picker time');
        }

        // Use the documented API: bulmaCalendar.attach returns a single instance, not an array.
        let calendarInstance = bulmaCalendar.attach(element, {
            type: picker_type || (String(time_format || '').trim() ? 'datetime' : 'date'),
            color: 'info',
            lang: 'en',
            dateFormat: date_format,
            timeFormat: time_format,
        });

        console.log('[Calendar] bulmaCalendar.attach returned instance', calendarInstance);

        if (!calendarInstance) {
            console.error('[Calendar] bulmaCalendar.attach did not return an instance for element', element.id);
            return;
        }

        init.set(element.id, calendarInstance);
        console.log('[Calendar] setup_date_picker: attached', {
            id: element.id,
            initial_date,
            date_format,
            time_format,
            picker_type,
            instanceKeys: Object.keys(calendarInstance || {})
        });

        // Helper: read current value from instance
        const readValue = (source) => {
            try {
                const v = calendarInstance.value();
                console.log('[Calendar] readValue from', source, '->', v);
                return v == null ? '' : String(v);
            } catch (e) {
                console.warn('[Calendar] readValue failed from', source, e);
                return '';
            }
        };

        // Normal selection: user picks a date/time
        calendarInstance.on('select', function(_datepicker) {
            const v = readValue('select');
            console.log('[Calendar] event: select -> calling on_change with', v);
            on_change(v);
        });

        // Validation event (e.g., user confirms selection)
        calendarInstance.on('validate', function(_datepicker) {
            const v = readValue('validate');
            console.log('[Calendar] event: validate -> calling on_change with', v);
            on_change(v);
        });

        // Clear via clear button
        calendarInstance.on('clear', function(_datepicker) {
            console.log('[Calendar] event: clear -> calling on_change with empty string');
            on_change('');
        });

        // Cancel button in datetime picker (bulma-calendar specific event)
        calendarInstance.on('onCancelClickDateTimePicker', function(_dp) {
            console.log('[Calendar] event: onCancelClickDateTimePicker -> calling on_change with empty string');
            on_change('');
        });

        // Today button click in datetime picker (bulma-calendar specific event)
        calendarInstance.on('onTodayClickDateTimePicker', function(_dp) {
            const v = readValue('onTodayClickDateTimePicker');
            console.log('[Calendar] event: onTodayClickDateTimePicker -> calling on_change with', v);
            on_change(v);
        });

        // Generic "today" event (older/newer versions)
        calendarInstance.on('today',  function(_dp) {
            const v = readValue('today');
            console.log('[Calendar] event: today -> calling on_change with', v);
            on_change(v);
        });

        // As a robust fallback across versions/skins, bind a delegated listener to the Today button
        // We scope it to this calendar instance using the closest calendar container with data-id
        const delegate = function(e) {
            const t = e.target;
            if (!t || !(t instanceof Element)) return;
            const btn = t.closest('[data-action="today"], .datetimepicker-today, .datepicker-footer-today, .bulma-calendar .is-today');
            if (!btn) return;
            // Try to identify the calendar container to match against this element.id
            const container = btn.closest('.datetimepicker, .bulma-calendar, .datepicker, .timepicker');
            const dataId = container?.getAttribute('data-id') || container?.dataset?.id;
            const idToMatch = dataId || (container?.getAttribute('for')) || null;
            // If we could determine an id and it doesn't match this element, ignore
            if (idToMatch && idToMatch !== element.id) return;
            // Otherwise, assume it's for this element (common when only one open at a time)
            const v = readValue('today-delegated');
            console.log('[Calendar] event: today (delegated) -> calling on_change with', v, { id: element.id });
            try { on_change(v); } catch (err) { console.warn('[Calendar] today (delegated) callback error', err); }
        };
        if (!window.__lbcCalendarTodayDelegates) {
            window.__lbcCalendarTodayDelegates = new Map();
        }
        if (!window.__lbcCalendarTodayDelegates.has(element.id)) {
            document.addEventListener('click', delegate, true);
            window.__lbcCalendarTodayDelegates.set(element.id, delegate);
            console.log('[Calendar] today delegated listener bound', { id: element.id });
        }

        // Additionally, when popup opens, try to bind directly to the Today button if we can find it
        calendarInstance.on('open', (_dp) => {
            console.log('[Calendar] event: open for', element.id);
            // Delay to allow DOM to render
            setTimeout(() => {
                const root = document.querySelector(`.datetimepicker[data-id="${element.id}"]`) 
                          || document.querySelector(`.bulma-calendar[data-id="${element.id}"]`);
                const todaySelector = '[data-action="today"], .datetimepicker-today, .datepicker-footer-today, .bulma-calendar .is-today, .datetimepicker-footer .button.is-today, .datepicker-footer .button.is-today, button.is-today';
                const todayBtn = root?.querySelector(todaySelector);
                console.log('[Calendar] open: resolved root and todayBtn', { root, todayBtn });
                if (todayBtn) {
                    const directHandler = () => {
                        // Read value after bulma updates selection
                        setTimeout(() => {
                            const v = readValue('today-direct');
                            console.log('[Calendar] event: today (direct) -> calling on_change with', v, { id: element.id });
                            try { on_change(v); } catch (err) { console.warn('[Calendar] today (direct) callback error', err); }
                        }, 0);
                    };
                    todayBtn.addEventListener('click', directHandler);
                    // Store handler on element so GC can clean when DOM removed; no strong refs kept globally
                    todayBtn._lbcTodayHandler = directHandler; // harmless custom prop
                    console.log('[Calendar] bound direct Today button handler', { id: element.id });
                } else {
                    console.warn('[Calendar] Today button not found on open', { id: element.id });
                }

                // As an extra safeguard: capture clicks inside the popup root and detect a Today-like button
                if (root && !root._lbcTodayRootHandler) {
                    const containerCapture = (e) => {
                        const t = e.target;
                        if (!t || !(t instanceof Element)) return;
                        let btn = t.closest(todaySelector);
                        // If selector didn't match, try text-based detection for visible buttons
                        if (!btn) {
                            const candidate = t.closest('button, .button, [role="button"]');
                            const label = (candidate?.textContent || '').trim().toLowerCase();
                            const todayWords = ['today','сегодня','heute','oggi','aujourd\'hui','hoy','hoje'];
                            if (candidate && todayWords.includes(label)) {
                                btn = candidate;
                            }
                        }
                        if (!btn) return;
                        // Defer read until after bulma processes the click
                        setTimeout(() => {
                            const v = readValue('today-container');
                            console.log('[Calendar] event: today (container) -> calling on_change with', v, { id: element.id });
                            try { on_change(v); } catch (err) { console.warn('[Calendar] today (container) callback error', err); }
                        }, 0);
                    };
                    root.addEventListener('click', containerCapture, true);
                    root._lbcTodayRootHandler = containerCapture;
                    console.log('[Calendar] today container listener bound', { id: element.id });
                }
            }, 0);
        });

        // On close, remove container capture listener if present
        calendarInstance.on('close', (_dp) => {
            console.log('[Calendar] event: close for', element.id);
            const root = document.querySelector(`.datetimepicker[data-id="${element.id}"]`) 
                      || document.querySelector(`.bulma-calendar[data-id="${element.id}"]`);
            if (root && root._lbcTodayRootHandler) {
                try { root.removeEventListener('click', root._lbcTodayRootHandler, true); } catch (_) {}
                try { delete root._lbcTodayRootHandler; } catch (_) { root._lbcTodayRootHandler = undefined; }
                console.log('[Calendar] today container listener removed', { id: element.id });
            }
        });
    } else {
        console.log('[Calendar] setup_date_picker: instance already initialized for', element.id);
    }

    // Evaluate and log current datepicker state right after init
    try {
        const inst = init.get(element.id);
        if (inst) {
            const currentValue = inst.value();
            console.log('[Calendar] post-init state', {
                id: element.id,
                currentValue,
                hasData: !!inst.data,
                dataKeys: inst.data ? Object.keys(inst.data) : []
            });
        } else {
            console.warn('[Calendar] post-init: no instance found in init map for', element.id);
        }
    } catch (e) {
        console.warn('[Calendar] post-init evaluation failed for', element.id, e);
    }

    // Set initial value if provided
    try {
        const inst = init.get(element.id);
        if (inst && typeof inst.value === 'function') {
            console.log('[Calendar] setting initial value via instance.value()', initial_date);
            inst.value(initial_date);
        } else {
            console.warn('[Calendar] instance.value is not a function for', element.id, inst);
        }
    } catch (e) {
        console.warn('[Calendar] failed to set initial value for', element.id, e);
    }
}

/**
 * Detach bulmaCalendar and any delegated listeners for the given id.
 */
export function detach_date_picker(id) {
    try {
        const key = typeof id === 'string' ? id : id?.toString?.() || String(id);
        console.log('[Calendar] detach_date_picker called for', key);
        if (window.__lbcCalendarTodayDelegates && window.__lbcCalendarTodayDelegates.has(key)) {
            const delegate = window.__lbcCalendarTodayDelegates.get(key);
            if (delegate) document.removeEventListener('click', delegate, true);
            window.__lbcCalendarTodayDelegates.delete(key);
            console.log('[Calendar] today delegated listener removed', { id: key });
        }
    } catch (e) {
        console.warn('[Calendar] detach_date_picker error', e);
    }
    init.delete(id);
}
"#)]
#[cfg(target_arch = "wasm32")]
#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn setup_date_picker(
        element: &Element,
        on_change: &JsValue,
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
                date="2025-02-03 12:34".to_string()
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
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use std::sync::Arc;
    use wasm_bindgen_test::*;

    fn noop() -> Arc<dyn Fn(String) + Send + Sync> {
        Arc::new(|_| {})
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
