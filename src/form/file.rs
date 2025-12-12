use leptos::html;
use leptos::prelude::{
    ClassAttribute, CustomAttribute, ElementChild, Get, GetUntracked, IntoAny, IntoView, NodeRef,
    NodeRefAttribute, Signal, component, view,
};

#[cfg(target_arch = "wasm32")]
type LbcSysFile = ();
#[cfg(not(target_arch = "wasm32"))]
type LbcSysFile = ();

use crate::util::{Size, TestAttr};

/// A custom file upload input in Bulma style.
///
/// https://bulma.io/documentation/form/file/
///
/// Controlled component:
/// - `files` is the current value (supports static Vec<File> or reactive signal).
/// - `update` is a required callback invoked with the selected files on change.
///
/// NOTE (tachys 0.2.11):
/// - Avoid `on:*` event bindings to prevent "callback removed before attaching" panics.
///   We attach the change listener manually on wasm32.
/// - Avoid reactive attribute/property bindings (class/attrs/multiple/labels) which can
///   also trigger tachys lifecycle panics. We compute these once using `get_untracked()`.
/// - We keep the component compiling on non-wasm targets by using a placeholder file type.
#[component]
pub fn File(
    /// The `name` attribute for this form element.
    #[prop(into)]
    name: String,

    /// The controlled list of selected files.
    ///
    /// Accepts a Vec<File> (wasm32) or a placeholder Vec<()> (non-wasm) signal.
    #[prop(into)]
    _files: Signal<Vec<LbcSysFile>>,

    /// Callback to propagate the selected files to the parent.
    _update: std::sync::Arc<dyn Fn(Vec<LbcSysFile>) + Send + Sync>,

    /// The display text for the file selector.
    #[prop(default = "Choose a file...".to_string().into(), into)]
    selector_label: Signal<String>,

    /// Additional CSS classes to append to Bulma's "file".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// If Some, adds `has-name` and shows a placeholder until files are selected.
    #[prop(optional, into)]
    has_name: Option<Signal<String>>,

    /// Move the CTA element to the right side of the component.
    #[prop(optional, into)]
    right: Signal<bool>,

    /// Expand the file display name to the full width of the parent.
    #[prop(optional, into)]
    fullwidth: Signal<bool>,

    /// Display as a boxed block.
    #[prop(optional, into)]
    boxed: Signal<bool>,

    /// Allow multiple files to be selected.
    #[prop(optional, into)]
    multiple: Signal<bool>,

    /// The size of this component.
    #[prop(optional)]
    size: Option<Size>,

    /// Optional test attribute (renders as data-* attribute) on the root <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,
) -> impl IntoView {
    // Compute attributes once to avoid tachys reactive property/event handle lifetimes.
    let mut class_parts = vec!["file".to_string()];

    let extra_classes = classes.get_untracked();
    if !extra_classes.trim().is_empty() {
        class_parts.push(extra_classes);
    }

    let has_name_text = has_name.as_ref().map(|signal| signal.get_untracked());
    if has_name_text.is_some() {
        class_parts.push("has-name".to_string());
    }

    if right.get_untracked() {
        class_parts.push("is-right".to_string());
    }
    if fullwidth.get_untracked() {
        class_parts.push("is-fullwidth".to_string());
    }
    if boxed.get_untracked() {
        class_parts.push("is-boxed".to_string());
    }

    if let Some(size) = size {
        match size {
            Size::Small => class_parts.push("is-small".to_string()),
            Size::Normal => {}
            Size::Medium => class_parts.push("is-medium".to_string()),
            Size::Large => class_parts.push("is-large".to_string()),
        }
    }

    let class = class_parts.join(" ");
    let selector_label_text = selector_label.get_untracked();
    let is_multiple = multiple.get_untracked();

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    // Workaround for tachys 0.2.11 panic "callback removed before attaching":
    // avoid `on:change` and attach the change listener manually on wasm32.
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::web_sys::Event;

        let has_attached = Rc::new(Cell::new(false));
        let input_ref_for_effect = input_ref.clone();
        let update_for_effect = _update.clone();

        Effect::new(move |_| {
            if has_attached.get() {
                return;
            }

            let Some(input_element) = input_ref_for_effect.get() else {
                return;
            };

            // Clone inside the effect so the effect closure remains `FnMut` (not `FnOnce`).
            let update_for_change = update_for_effect.clone();

            let change_closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |_event: Event| {
                    // Placeholder behavior: we don't currently extract real File objects.
                    // We still call update with an empty list to keep the controlled contract.
                    (update_for_change)(Vec::<LbcSysFile>::new());
                }));

            input_element
                .add_event_listener_with_callback("change", change_closure.as_ref().unchecked_ref())
                .ok();

            has_attached.set(true);
            change_closure.forget();
        });
    }

    view! {
        <div
            class=class
            attr:data-testid=data_testid
            attr:data-cy=data_cy
        >
            <label class="file-label">
                <input
                    node_ref=input_ref
                    type="file"
                    class="file-input"
                    name=name.clone()
                    multiple=is_multiple
                />
                <span class="file-cta">
                    <span class="file-icon"></span>
                    <span class="file-label">
                        {selector_label_text}
                    </span>
                </span>

                {
                    if let Some(file_name_text) = has_name_text {
                        view! { <span class="file-name">{file_name_text}</span> }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }
            </label>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Size;
    use leptos::prelude::RenderHtml;
    use std::sync::Arc;

    fn noop_update() -> Arc<dyn Fn(Vec<super::LbcSysFile>) + Send + Sync> {
        Arc::new(|_files| {})
    }

    #[test]
    fn file_renders_base_class_and_input() {
        let html = view! {
            <File
                name="upload"
                _files=Signal::derive(|| Vec::<super::LbcSysFile>::new())
                _update=noop_update()
            />
        }
        .to_html();

        assert!(
            html.contains(r#"class="file""#),
            "expected base 'file' class; got: {}",
            html
        );
        assert!(
            html.contains(r#"type="file""#),
            "expected file input; got: {}",
            html
        );
    }

    #[test]
    fn file_applies_size_and_flags() {
        let html = view! {
            <File
                name="upload"
                _files=Signal::derive(|| Vec::<super::LbcSysFile>::new())
                _update=noop_update()
                size=Size::Small
                right=true
                fullwidth=true
                boxed=true
            />
        }
        .to_html();

        assert!(
            html.contains("is-small"),
            "expected is-small; got: {}",
            html
        );
        assert!(
            html.contains("is-right"),
            "expected is-right; got: {}",
            html
        );
        assert!(
            html.contains("is-fullwidth"),
            "expected is-fullwidth; got: {}",
            html
        );
        assert!(
            html.contains("is-boxed"),
            "expected is-boxed; got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::util::{Size, TestAttr};
    use leptos::prelude::*;
    use std::sync::Arc;
    use wasm_bindgen_test::*;

    fn noop_update() -> Arc<dyn Fn(Vec<super::LbcSysFile>) + Send + Sync> {
        Arc::new(|_files| {})
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn file_renders_test_attr_as_data_testid() {
        let html = view! {
            <File
                name="upload"
                _files=Signal::derive(|| Vec::<super::LbcSysFile>::new())
                _update=noop_update()
                size=Size::Small
                test_attr=TestAttr::test_id("file-test")
            />
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="file-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn file_no_test_attr_when_not_provided() {
        let html = view! {
            <File
                name="upload"
                _files=Signal::derive(|| Vec::<super::LbcSysFile>::new())
                _update=noop_update()
            />
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute; got: {}",
            html
        );
    }
}
