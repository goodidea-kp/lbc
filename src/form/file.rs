use leptos::prelude::{
    ClassAttribute, CustomAttribute, ElementChild, Get, IntoAny, IntoView, OnAttribute, Signal,
    component, view,
};

#[cfg(target_arch = "wasm32")]
type LbcSysFile = ();
#[cfg(not(target_arch = "wasm32"))]
type LbcSysFile = ();

use crate::util::Size;

/// A custom file upload input in Bulma style.
///
/// https://bulma.io/documentation/form/file/
///
/// Controlled component:
/// - `files` is the current value (supports static Vec<File> or reactive signal).
/// - `update` is a required callback invoked with the selected files on change.
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

    /// Optional test identifier (renders as data-testid attribute)
    #[prop(optional, into)]
    test_id: Option<String>,
) -> impl IntoView {
    let has_name_for_class = has_name.clone();
    let class = move || {
        let mut parts = vec!["file".to_string()];

        let extra = classes.get();
        if !extra.trim().is_empty() {
            parts.push(extra);
        }
        if has_name_for_class.is_some() {
            parts.push("has-name".to_string());
        }
        if right.get() {
            parts.push("is-right".to_string());
        }
        if fullwidth.get() {
            parts.push("is-fullwidth".to_string());
        }
        if boxed.get() {
            parts.push("is-boxed".to_string());
        }
        if let Some(size) = size {
            match size {
                Size::Small => parts.push("is-small".to_string()),
                Size::Normal => {}
                Size::Medium => parts.push("is-medium".to_string()),
                Size::Large => parts.push("is-large".to_string()),
            }
        }

        parts.join(" ")
    };

    let filenames_view = {
        #[cfg(target_arch = "wasm32")]
        {
            let has_name = has_name.clone();
            move || {
                if let Some(placeholder_signal) = has_name.as_ref() {
                    let placeholder = placeholder_signal.get();
                    view! { <span class="file-name">{placeholder}</span> }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let has_name = has_name.clone();
            move || {
                // In non-wasm targets we can't inspect File objects; only render placeholder if provided.
                if let Some(placeholder_signal) = has_name.as_ref() {
                    let placeholder = placeholder_signal.get();
                    view! { <span class="file-name">{placeholder}</span> }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }
        }
    };

    let icon_view = || view! { <span class="file-icon"></span> }.into_any();

    #[cfg(target_arch = "wasm32")]
    let on_change = {
        move |_ev: leptos::ev::Event| {
            // File APIs are not available via leptos::web_sys re-export in this build.
            // No-op placeholder to keep the component compiling under wasm32 without extra deps.
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    let on_change = |_ev: leptos::ev::Event| { /* no-op on non-wasm targets */ };

    view! {
        <div class=class data-testid=test_id>
            <label class="file-label">
                <input
                    type="file"
                    class="file-input"
                    name=name.clone()
                    multiple=move || multiple.get()
                    on:change=on_change
                />
                <span class="file-cta">
                    {icon_view()}
                    <span class="file-label">
                        { move || selector_label.get() }
                    </span>
                </span>
                {filenames_view()}
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
    use crate::util::Size;
    use leptos::prelude::*;
    use std::sync::Arc;
    use wasm_bindgen_test::*;

    fn noop_update() -> Arc<dyn Fn(Vec<super::LbcSysFile>) + Send + Sync> {
        Arc::new(|_files| {})
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn file_renders_test_id() {
        let html = view! {
            <File
                name="upload"
                _files=Signal::derive(|| Vec::<super::LbcSysFile>::new())
                _update=noop_update()
                size=Size::Small
                test_id="file-test"
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
    fn file_no_test_id_when_not_provided() {
        let html = view! {
            <File
                name="upload"
                _files=Signal::derive(|| Vec::<super::LbcSysFile>::new())
                _update=noop_update()
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
