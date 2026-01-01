use leptos::callback::Callback;
#[allow(unused_imports)]
use leptos::prelude::Effect;
use leptos::prelude::{
    Callable, ClassAttribute, CustomAttribute, ElementChild, GetUntracked, IntoAny, IntoView,
    OnAttribute, Signal, component, view,
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
    _update: Callback<Vec<LbcSysFile>>,

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

    // NOTE: LbcSysFile is currently `()` in both wasm32 and non-wasm builds in this codebase.
    // We still wire the change handler so the controlled pattern works and can be upgraded later
    // when a real file type is introduced.
    let on_change = {
        let update = _update.clone();
        move |_ev: leptos::web_sys::Event| {
            #[cfg(target_arch = "wasm32")]
            {
                // We can read the FileList length, but we don't have a stable cross-platform file type
                // in this crate yet (LbcSysFile is `()`), so we pass a Vec<()> of the same length.
                use leptos::wasm_bindgen::JsCast;
                let target = _ev
                    .target()
                    .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok());

                let mut out: Vec<LbcSysFile> = Vec::new();
                if let Some(input) = target {
                    if let Some(files) = input.files() {
                        let len = files.length() as usize;
                        out = vec![(); len];
                    }
                }
                update.run(out);
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                // No DOM / File API available; just notify with empty selection.
                update.run(Vec::new());
            }
        }
    };

    view! {
        <div
            class=class
            attr:data-testid=data_testid
            attr:data-cy=data_cy
        >
            <label class="file-label">
                <input
                    type="file"
                    class="file-input"
                    name=name.clone()
                    multiple=is_multiple
                    on:change=on_change
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

    fn noop_update() -> Callback<Vec<super::LbcSysFile>> {
        Callback::new(|_files: Vec<super::LbcSysFile>| {})
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
    use wasm_bindgen_test::*;

    fn noop_update() -> Callback<Vec<super::LbcSysFile>> {
        Callback::new(|_files: Vec<super::LbcSysFile>| {})
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
