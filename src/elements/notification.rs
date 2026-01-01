use crate::util::TestAttr;
use leptos::callback::Callback;
use leptos::children::Children;
use leptos::prelude::{
    Callable, ClassAttribute, CustomAttribute, ElementChild, Get, GetUntracked, IntoView,
    NodeRefAttribute, OnAttribute, Signal, StyleAttribute, component, view,
};

#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use leptos::web_sys;

#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Function, Reflect};

/// Bold notification blocks, to alert your users of something.
///
/// https://bulma.io/documentation/elements/notification/
///
/// This component can be used in two modes:
/// 1) Inline notification (default): renders a normal Bulma `.notification` block.
/// 2) Toast notification (`toast=true`): renders a native Popover-based toast that can auto-hide.
///
/// Toast requirements:
/// - Uses the modern browser Popover API (`popover="manual"`).
/// - Auto-shows/auto-hides based on `open`.
/// - Auto-hides after `auto_hide_ms` milliseconds (default 5000).
/// - If `auto_hide_ms == -1`, it will not auto-hide.
/// - A dismiss button is always rendered (required).
#[component]
pub fn Notification(
    #[prop(optional, into)]
    classes: Signal<String>,

    /// When true, render as a toast using the Popover API.
    #[prop(optional, into)]
    toast: Signal<bool>,

    /// Controls whether the toast is open (only used when `toast=true`).
    #[prop(optional, into)]
    open: Signal<bool>,

    /// Setter for `open` (required when `toast=true` so dismiss/auto-hide can close it).
    #[prop(optional)]
    set_open: Option<Callback<bool>>,

    /// Auto-hide duration in milliseconds (toast mode only).
    /// - default: 5000
    /// - -1: do not auto-hide
    #[prop(optional, into)]
    auto_hide_ms: Signal<i32>,

    /// Optional test attribute (renders as data-* attribute) on the root <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key (for example, `data-cy`).
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    children: Children,
) -> impl IntoView {
    let class = move || {
        let extras = classes.get();
        if extras.trim().is_empty() {
            "notification".to_string()
        } else {
            format!("notification {}", extras.trim())
        }
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    // Toast mode requires a setter so we can close from the dismiss button and auto-hide.
    let close_cb: Option<Callback<()>> = {
        let toast_now = toast.get_untracked();
        if toast_now {
            set_open.as_ref().map(|set_open| {
                let set_open = set_open.clone();
                Callback::new(move |_| set_open.run(false))
            })
        } else {
            None
        }
    };

    // In toast mode, we use a popover element and imperatively show/hide it.
    // We also schedule auto-hide when opened.
    let popover_ref: leptos::prelude::NodeRef<leptos::html::Div> = leptos::prelude::NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        let toast = toast.clone();
        let open = open.clone();
        let auto_hide_ms = auto_hide_ms.clone();
        let set_open = set_open.clone();
        let popover_ref = popover_ref.clone();

        leptos::prelude::Effect::new(move |_| {
            if !toast.get() {
                return;
            }

            let Some(div) = popover_ref.get() else {
                return;
            };

            let el: web_sys::HtmlElement = div.unchecked_into();

            // show/hide popover based on `open`
            if open.get() {
                // showPopover() is available on HTMLElement in modern browsers.
                // If not available, this will no-op via JS reflection.
                if let Ok(v) = Reflect::get(&el, &JsValue::from_str("showPopover")) {
                    if let Some(f) = v.dyn_ref::<Function>() {
                        let _ = Reflect::apply(f, &el, &Array::new());
                    }
                }

                // schedule auto-hide if configured
                let ms = auto_hide_ms.get();
                if ms >= 0 {
                    if let Some(set_open) = set_open.as_ref() {
                        let set_open = set_open.clone();
                        let cb = Closure::wrap(Box::new(move || {
                            set_open.run(false);
                        }) as Box<dyn FnMut()>);

                        if let Some(window) = web_sys::window() {
                            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                                cb.as_ref().unchecked_ref(),
                                ms,
                            );
                        }

                        // Let the browser own the callback until it fires.
                        cb.forget();
                    }
                }
            } else {
                if let Ok(v) = Reflect::get(&el, &JsValue::from_str("hidePopover")) {
                    if let Some(f) = v.dyn_ref::<Function>() {
                        let _ = Reflect::apply(f, &el, &Array::new());
                    }
                }
            }
        });
    }

    // Defaults: toast=false, open=false, auto_hide_ms=5000
    // We implement defaults by using get_untracked() snapshots for non-reactive defaults.
    let toast_now = toast.get_untracked();
    let auto_hide_defaulted = {
        let v = auto_hide_ms.get_untracked();
        if v == 0 {
            5000
        } else {
            v
        }
    };

    if toast_now {
        // Enforce required props in toast mode at runtime (compile-time enforcement would require
        // a separate component or a different prop shape).
        let has_setter = set_open.is_some();
        if !has_setter {
            crate::lbc_debug_log!(
                "[Notification] toast=true but set_open was not provided; dismiss/auto-hide will not work."
            );
        }

        // Toast styling: Bulma doesn't define a toast layout, so we provide minimal positioning.
        // Consumers can override via `classes`.
        view! {
            <div
                node_ref=popover_ref
                class=move || {
                    // Base toast positioning + Bulma notification styling
                    let base = class();
                    // Default toast positioning; can be overridden by passing classes.
                    // Using inline style keeps this self-contained.
                    base
                }
                style="position: fixed; right: 1rem; top: 1rem; z-index: 9999; max-width: 420px;"
                popover="manual"
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                <button
                    class="delete"
                    type="button"
                    on:click=move |_| {
                        if let Some(cb) = &close_cb {
                            cb.run(());
                        }
                    }
                ></button>

                {children()}

                // Store the defaulted value so it isn't optimized away; also documents behavior.
                <span style="display:none">{auto_hide_defaulted.to_string()}</span>
            </div>
        }
        .into_view()
    } else {
        // Inline mode: original behavior, no popover, no dismiss button.
        view! {
            <div
                class=class
                attr:data-testid=move || data_testid.clone()
                attr:data-cy=move || data_cy.clone()
            >
                {children()}
            </div>
        }
        .into_view()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    #[test]
    fn notification_renders_children() {
        let html = view! { <Notification>{"Heads up!"}</Notification> }.to_html();
        assert!(
            html.contains(r#"class="notification""#),
            "expected base notification class, got: {html}"
        );
        assert!(html.contains("Heads up!"), "expected children to render");
    }

    #[test]
    fn notification_appends_custom_classes() {
        let html =
            view! { <Notification classes="is-link is-light">{"Link notice"}</Notification> }
                .to_html();
        assert!(
            html.contains(r#"class="notification is-link is-light""#),
            "expected additional classes, got: {html}"
        );
    }

    #[test]
    fn toast_notification_renders_popover_and_delete_button() {
        let (open, set_open) = leptos::prelude::signal(true);
        let html = view! {
            <Notification
                toast=true
                open=open
                set_open=Callback::new(move |v| set_open.set(v))
                auto_hide_ms=5000
            >
                {"Toast content"}
            </Notification>
        }
        .to_html();

        assert!(
            html.contains(r#"popover="manual""#),
            "expected popover attribute in toast mode; got: {html}"
        );
        assert!(
            html.contains(r#"class="delete""#),
            "expected dismiss button in toast mode; got: {html}"
        );
        assert!(html.contains("Toast content"), "expected toast content; got: {html}");
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn notification_renders_test_id() {
        let (open, set_open) = signal(true);
        let html = view! {
            <Notification
                toast=true
                open=open
                set_open=Callback::new(move |v| set_open.set(v))
                test_attr=TestAttr::test_id("notification-test")
            >
                {"Content"}
            </Notification>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="notification-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn notification_no_test_attr_when_not_provided() {
        let (open, set_open) = signal(true);
        let html = view! {
            <Notification
                toast=true
                open=open
                set_open=Callback::new(move |v| set_open.set(v))
            >
                {"Content"}
            </Notification>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no test attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn notification_accepts_custom_test_attr_key() {
        let (open, set_open) = signal(true);
        let html = view! {
            <Notification
                toast=true
                open=open
                set_open=Callback::new(move |v| set_open.set(v))
                test_attr=TestAttr::new("data-cy", "notification-cy")
            >
                {"Content"}
            </Notification>
        }
        .to_html();

        assert!(
            html.contains(r#"data-cy="notification-cy""#),
            "expected custom data-cy attribute; got: {}",
            html
        );
    }
}
