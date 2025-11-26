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
    component, view, ClassAttribute, CustomAttribute, Get, GetUntracked, GlobalAttributes, IntoView,
    NodeRef, NodeRefAttribute, Signal,
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
#[cfg(target_arch = "wasm32")]
#[leptos::wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
let init = new Map();

/**
 * Attach bulma-calendar to the given input element and wire native DOM events
 * back to the provided callback.
 *
 * We do NOT rely on calendarInstance.on(...) because the CDN build of
 * bulma-calendar@7.1.1 does not expose a stable .on API. Instead we:
 *   - listen to the input's native 'change' event
 *   - best-effort hook the Today button's click
 */
export function setup_date_picker(element, callback, initial_date, date_format, time_format, picker_type) {
    // Only initialize once per element id
    if (!init.has(element.id)) {
        // Attach bulma-calendar to the input element
        let calendarInstances = bulmaCalendar.attach(element, {
            type: picker_type || (String(time_format || '').trim() ? 'datetime' : 'date'),
            color: 'info',
            lang: 'en',
            dateFormat: date_format,
            timeFormat: time_format,
        });

        // Normalize instance (array vs single)
        let calendarInstance = Array.isArray(calendarInstances) ? calendarInstances[0] : calendarInstances;
        init.set(element.id, calendarInstance);

        // 1) Native input change event: fires when bulma-calendar updates the value
        element.addEventListener('change', function () {
            const value = element.value || '';
            console.debug('bulma-calendar input change →', value);
            callback(value);
        });

        // 2) Best-effort Today button hook
        try {
            // The calendar popup is usually rendered as a sibling or descendant.
            // We search the document for a calendar container associated with this input.
            // This is heuristic but works for the catalog.
            const doc = element.ownerDocument || document;
            // Look for a calendar container that references this input by id
            // or is near it in the DOM.
            let container = null;

            // Strategy A: data-target attribute
            container = doc.querySelector(
                '.datetimepicker[data-target="#' + element.id + '"]'
            );

            // Strategy B: first datetimepicker next to the input
            if (!container) {
                const candidates = doc.querySelectorAll('.datetimepicker');
                if (candidates.length === 1) {
                    container = candidates[0];
                } else if (candidates.length > 1) {
                    // pick the one closest in the DOM tree
                    let best = null;
                    let bestDistance = Infinity;
                    candidates.forEach(function (node) {
                        let distance = 0;
                        let current = node;
                        while (current && current !== element && distance < 10) {
                            current = current.parentElement;
                            distance++;
                        }
                        if (current === element && distance < bestDistance) {
                            best = node;
                            bestDistance = distance;
                        }
                    });
                    if (best) container = best;
                }
            }

            if (container && container.querySelector) {
                // Today button usually has data-action="today" or a specific class
                const todayButton =
                    container.querySelector('[data-action="today"]') ||
                    container.querySelector('.datetimepicker-today') ||
                    container.querySelector('.is-today');

                if (todayButton) {
                    todayButton.addEventListener('click', function () {
                        // Let bulma-calendar update its internal state and the input value first
                        setTimeout(function () {
                            const value = element.value || '';
                            console.debug('bulma-calendar Today click →', value);

                            // Manually dispatch a native 'change' event so our input listener runs.
                            try {
                                const evt = new Event('change', { bubbles: true });
                                element.dispatchEvent(evt);
                            } catch (e) {
                                console.warn('bulma-calendar: failed to dispatch synthetic change event', e);
                                // Fallback: call callback directly if dispatch fails
                                callback(value);
                            }
                        }, 0);
                    });
                } else {
                    console.debug('bulma-calendar: Today button not found for element id=', element.id);
                }
            } else {
                console.debug('bulma-calendar: calendar container not found for element id=', element.id);
            }
        } catch (e) {
            console.warn('bulma-calendar: failed to hook Today button', e);
        }
    }

    // Set initial value on the input; bulma-calendar will pick it up
    if (typeof initial_date === 'string') {
        element.value = initial_date;
    } else if (initial_date && typeof initial_date.toString === 'function') {
        element.value = initial_date.toString();
    }
}

/**
 * Detach bookkeeping for the given id.
 * (We do not destroy the JS widget explicitly; bulma-calendar v7 does not
 * expose a stable destroy API in the CDN build.)
 */
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
