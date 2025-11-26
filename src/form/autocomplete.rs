/*!
AutoComplete component: a Leptos wrapper around the Bulma Tags Input plugin.

Required static assets
- Add the Bulma TagsInput CSS into your HTML <head>:
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@creativebulma/bulma-tagsinput@1.0.3/dist/css/bulma-tagsinput.min.css" />

- Add the Bulma TagsInput JS so `BulmaTagsInput` is available on window. Place this before your wasm bootstrap script
  (or ensure it loads before your app mounts):
  <script src="https://cdn.jsdelivr.net/npm/@creativebulma/bulma-tagsinput@1.0.3/dist/js/bulma-tagsinput.min.js"></script>

Notes
- We attach the JS behavior on mount and detach on unmount.
- SSR tests only verify the rendered HTML structure.
*/

use leptos::prelude::{
    ClassAttribute, CustomAttribute, ElementChild, Get, GlobalAttributes, IntoAny, IntoView,
    Signal, component, view,
};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use leptos::web_sys::Element;

/// A tags autocomplete input based on Bulma TagsInput.
///
/// Two modes:
/// - Static: provide `items` (and leave `data_item_*` empty) to render a <select data-type="tags"> with options.
/// - Dynamic: provide both `data_item_text` and `data_item_value` plus `url_for_fetch` for async suggestions.
#[component]
pub fn AutoComplete(
    /// Unique DOM id for the element used by the JS plugin.
    id: String,

    /// Maximum number of tags allowed (default 10).
    #[prop(optional)]
    max_items: Option<u32>,

    /// Static list of items. If provided and `data_item_text/value` are empty, renders a <select data-type="tags">.
    #[prop(optional)]
    items: Option<Vec<String>>,

    /// Called when a tag is added.
    _on_update: Arc<dyn Fn(String) + Send + Sync>,

    /// Called when a tag is removed.
    _on_remove: Arc<dyn Fn(String) + Send + Sync>,

    /// Currently selected single tag (for initial value).
    #[prop(optional, into)]
    current_selector: Signal<String>,

    /// Placeholder to show in the input/select.
    #[prop(optional, into)]
    placeholder: Signal<String>,

    /// Extra classes appended to the control.
    #[prop(optional, into)]
    classes: Signal<String>,

    /// Case sensitive matching.
    #[prop(optional, into)]
    _case_sensitive: Signal<bool>,

    /// For dynamic mode: object field to show as text.
    #[prop(optional, into)]
    data_item_text: Signal<String>,

    /// For dynamic mode: object field to use as value.
    #[prop(optional, into)]
    data_item_value: Signal<String>,

    /// For dynamic mode: base URL to fetch suggestions (the plugin appends the typed value).
    #[prop(optional, into)]
    _url_for_fetch: Signal<String>,

    /// Optional Authorization header value for dynamic fetches.
    #[prop(optional, into)]
    _auth_header: Signal<String>,

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
) -> impl IntoView {
    let _max_items_value = max_items.unwrap_or(10);

    // Build base input class
    let input_class = {
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

    // Render structure based on props.
    let static_mode = items.as_ref().map(|v| !v.is_empty()).unwrap_or(false)
        && data_item_text.get().trim().is_empty()
        && data_item_value.get().trim().is_empty();

    let body = if static_mode {
        let options_view = {
            let items = items.clone().unwrap_or_default();
            let current = current_selector.get();
            view! {
                <>
                    {items.into_iter().map(|item| {
                        let selected = item == current;
                        view! {
                            <option value=item.clone() selected=selected>{item.clone()}</option>
                        }.into_any()
                    }).collect::<Vec<_>>()}
                </>
            }
            .into_any()
        };
        view! {
            <div class=move || {
                let extra = classes.get();
                if extra.trim().is_empty() { "select".to_string() } else { format!("select {}", extra) }
            } data-testid=test_id.clone()>
                <select
                    id=id.clone()
                    data-type="tags"
                    data-placeholder=placeholder.get()
                >
                    {options_view}
                </select>
            </div>
        }
        .into_any()
    } else if !data_item_text.get().trim().is_empty() && !data_item_value.get().trim().is_empty() {
        // Dynamic object mode
        let value_json = {
            let current = current_selector.get();
            if current.trim().is_empty() {
                "{}".to_string()
            } else {
                format!("{{\"{}\":\"{}\"}}", data_item_value.get(), current)
            }
        };
        view! {
            <input
                id=id.clone()
                r#type="text"
                class=move || input_class()
                data-item-text=data_item_text.get()
                data-item-value=data_item_value.get()
                data-placeholder=placeholder.get()
                value=value_json
                data-testid=test_id.clone()
            />
        }
        .into_any()
    } else {
        // Plain text mode
        view! {
            <input
                id=id.clone()
                r#type="text"
                class=move || input_class()
                data-placeholder=placeholder.get()
                value=current_selector.get()
                data-testid=test_id.clone()
            />
        }
        .into_any()
    };

    // Attach JS plugin on mount.
    #[cfg(target_arch = "wasm32")]
    {
        let id_for_js = id.clone();
        let max_items = _max_items_value;
        let current_selector = current_selector.clone();
        let case_sensitive = _case_sensitive.clone();
        let url_for_fetch = _url_for_fetch.clone();
        let auth_header = _auth_header.clone();
        let data_item_value = data_item_value.clone();

        leptos::prelude::Effect::new(move |_| {
            let document = leptos::prelude::document();
            if let Some(element) = document.get_element_by_id(&id_for_js) {
                // Build callback to route ops to on_update/on_remove
                let cb = {
                    let on_update = _on_update.clone();
                    let on_remove = _on_remove.clone();
                    Closure::wrap(Box::new(move |json: JsValue| {
                        if let Some(s) = json.as_string() {
                            if let Some((op, value)) =
                                s.trim_matches(|c| c == '{' || c == '}').split_once(",")
                            {
                                // naive parse: "op":"add","value":"X"
                                let op_is_add = op.contains(r#""add""#);
                                let value_str = value.split(':').nth(1).unwrap_or("").trim();
                                let value_clean = value_str.trim_matches('"').to_string();
                                if op_is_add {
                                    (on_update)(value_clean);
                                } else {
                                    (on_remove)(value_clean);
                                }
                            }
                        }
                    }) as Box<dyn FnMut(JsValue)>)
                };

                let url = url_for_fetch.get();
                if url.trim().is_empty() {
                    // Static
                    setup_static_autocomplete(
                        &element.unchecked_into::<Element>(),
                        cb.as_ref(),
                        &JsValue::from(max_items),
                        &JsValue::from(case_sensitive.get()),
                    );
                } else {
                    // Dynamic
                    setup_dynamic_autocomplete(
                        &element.unchecked_into::<Element>(),
                        cb.as_ref(),
                        &JsValue::from(max_items),
                        &JsValue::from(url),
                        &JsValue::from(auth_header.get()),
                        &JsValue::from(case_sensitive.get()),
                        &JsValue::from(data_item_value.get()),
                        &JsValue::from(current_selector.get()),
                    );
                }
                cb.forget();
            }
        });

        // Cleanup detach
        let id_cleanup = id.clone();
        leptos::prelude::on_cleanup(move || {
            detach_autocomplete(&JsValue::from(id_cleanup.as_str()));
        });
    }

    view! { {body} }
}

// JS bridge similar to the Yew version, adapted for Leptos.
#[cfg(target_arch = "wasm32")]
#[leptos::wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
let init = new Map();
export function setup_dynamic_autocomplete(element, callback, max_tags, url_for_fetch, auth_header, case_sensitive, data_item_value, initial_value) {
     if (!init.has(element.id)) {
         let autocompleteInstance = BulmaTagsInput.attach(element, {
            maxTags: Number(max_tags) || 10,
            caseSensitive: !!case_sensitive,
            source: async function(value) {
                return await fetch(String(url_for_fetch) + value, {
                    headers: auth_header ? { 'Authorization': String(auth_header) } : {}
                })
                .then(function(response) {
                    if (response.status !== 200) {
                        throw new Error('Failed to fetch data');
                    }
                    return response.json();
                });
            },
         });
         let autocomplete = autocompleteInstance[0];
         autocomplete.on('after.add', function(tag) {
            callback('{"op":"add","value":"'+tag.item[data_item_value]+'"}');
         });
         autocomplete.on('after.remove', function(tag) {
            callback('{"op":"remove","value":"'+tag[data_item_value]+'"}');
         });
         if (String(initial_value).length > 0) {
            autocomplete.add('{"'+data_item_value+'":"'+String(initial_value)+'"}');
         }
         init.set(element.id, autocomplete);
     }
}

export function setup_static_autocomplete(element, callback, max_tags, case_sensitive) {
     if (!init.has(element.id)) {
         let autocompleteInstance = BulmaTagsInput.attach(element, {
            maxTags: Number(max_tags) || 10,
            caseSensitive: !!case_sensitive,
         });
         let autocomplete = autocompleteInstance[0];
         autocomplete.on('after.add', function(tag) {
            if (tag.item && tag.item.value) {
                callback('{"op":"add","value":"'+tag.item.value+'"}');
            } else if (tag.value) {
                callback('{"op":"add","value":"'+tag.value+'"}');
            } else {
                callback('{"op":"add","value":"'+tag.item+'"}');
            }
         });
         autocomplete.on('after.remove', function(tag) {
            if (tag.item && tag.item.value) {
                callback('{"op":"remove","value":"'+tag.item.value+'"}');
            } else if (tag.value) {
                callback('{"op":"remove","value":"'+tag.value+'"}');
            } else {
                callback('{"op":"remove","value":"'+tag+'"}');
            }
         });
         init.set(element.id, autocomplete);
     }
}

export function detach_autocomplete(id) {
   init.delete(String(id));
}
"#)]
#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn setup_dynamic_autocomplete(
        element: &Element,
        callback: &JsValue,
        max_tags: &JsValue,
        url_to_fetch: &JsValue,
        auth_header: &JsValue,
        case_sensitive: &JsValue,
        data_item_value: &JsValue,
        initial_value: &JsValue,
    );
    fn setup_static_autocomplete(
        element: &Element,
        callback: &JsValue,
        max_tags: &JsValue,
        case_sensitive: &JsValue,
    );
    fn detach_autocomplete(id: &JsValue);
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::RenderHtml;

    fn noop() -> Arc<dyn Fn(String) + Send + Sync> {
        Arc::new(|_| {})
    }

    #[test]
    fn renders_static_select_when_items_provided() {
        let html = view! {
            <AutoComplete
                id="ac1".to_string()
                items=vec!["A".to_string(), "B".to_string()]
                placeholder="Choose"
                _on_update=noop()
                _on_remove=noop()
            />
        }
        .to_html();

        assert!(
            html.contains(r#"data-type="tags""#),
            "expected tags select; got: {}",
            html
        );
        assert!(html.contains("<option"), "expected options; got: {}", html);
    }

    #[test]
    fn renders_dynamic_input_with_data_attrs() {
        let html = view! {
            <AutoComplete
                id="ac2".to_string()
                data_item_text="name"
                data_item_value="name"
                _url_for_fetch="/api?q="
                _on_update=noop()
                _on_remove=noop()
            />
        }
        .to_html();

        assert!(
            html.contains(r#"data-item-text="name""#),
            "expected data-item-text; got: {}",
            html
        );
        assert!(
            html.contains(r#"class="input""#),
            "expected input class; got: {}",
            html
        );
    }

    #[test]
    fn renders_plain_input_otherwise() {
        let html = view! {
            <AutoComplete
                id="ac3".to_string()
                placeholder="Type..."
                _on_update=noop()
                _on_remove=noop()
            />
        }
        .to_html();

        assert!(
            html.contains(r#"class="input""#) && html.contains(r#"id="ac3""#),
            "expected plain input; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use leptos::prelude::*;
    use std::sync::Arc;
    use wasm_bindgen_test::*;

    fn noop() -> Arc<dyn Fn(String) + Send + Sync> {
        Arc::new(|_| {})
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn autocomplete_renders_test_id_static_mode() {
        let html = view! {
            <AutoComplete
                id="ac1".to_string()
                items=vec!["A".to_string(), "B".to_string()]
                placeholder="Choose"
                _on_update=noop()
                _on_remove=noop()
                test_id="autocomplete-test"
            />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="autocomplete-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn autocomplete_no_test_id_when_not_provided() {
        let html = view! {
            <AutoComplete
                id="ac1".to_string()
                items=vec!["A".to_string(), "B".to_string()]
                placeholder="Choose"
                _on_update=noop()
                _on_remove=noop()
            />
        }
        .to_html();

        assert!(
            !html.contains("data-testid"),
            "expected no data-testid attribute; got: {}",
            html
        );
    }
}
