use leptos::prelude::{
    Children, ClassAttribute, CustomAttribute, ElementChild, Get, IntoAny, IntoView, Signal,
    component, view,
};

use crate::util::TestAttr;

/// Alignment options available for field addons (Bulma).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddonsAlign {
    Centered,
    Right,
}

impl AddonsAlign {
    fn bulma(self) -> &'static str {
        match self {
            AddonsAlign::Centered => "has-addons-centered",
            AddonsAlign::Right => "has-addons-right",
        }
    }
}

/// Alignment options available for grouped field controls (Bulma).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupedAlign {
    Centered,
    Right,
}

impl GroupedAlign {
    fn bulma(self) -> &'static str {
        match self {
            GroupedAlign::Centered => "is-grouped-centered",
            GroupedAlign::Right => "is-grouped-right",
        }
    }
}

/// The three sizes available for horizontal field labels (Bulma).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelSize {
    Small,
    Medium,
    Large,
}

impl LabelSize {
    fn bulma(self) -> &'static str {
        match self {
            LabelSize::Small => "is-small",
            LabelSize::Medium => "is-medium",
            LabelSize::Large => "is-large",
        }
    }
}

/// A container for form controls (Bulma "field").
///
/// Mirrors Bulma's field structure, including optional label and help,
/// addon/grouping modifiers, and horizontal layout support.
///
/// https://bulma.io/documentation/form/general/
#[component]
pub fn Field(
    /// Extra classes added to "field".
    #[prop(optional, into)]
    classes: Signal<String>,

    /// A text label for the field.
    #[prop(optional, into)]
    label: Option<Signal<String>>,

    /// Extra classes for the label container or label element.
    #[prop(optional, into)]
    label_classes: Signal<String>,

    /// A help message displayed under/after the body.
    #[prop(optional, into)]
    help: Option<Signal<String>>,

    /// Extra classes for the help container.
    #[prop(optional, into)]
    help_classes: Signal<String>,

    /// Convenience flag adding "is-danger" to help classes.
    #[prop(optional, into)]
    help_has_error: Signal<bool>,

    /// Has icons on the left of the field's controls.
    #[prop(optional, into)]
    icons_left: Signal<bool>,

    /// Has icons on the right of the field's controls.
    #[prop(optional, into)]
    icons_right: Signal<bool>,

    /// Allow addons to the field's controls.
    #[prop(optional, into)]
    addons: Signal<bool>,

    /// Alignment for the field's addons.
    #[prop(optional)]
    addons_align: Option<AddonsAlign>,

    /// All controls in this field should be grouped.
    #[prop(optional, into)]
    grouped: Signal<bool>,

    /// Alignment for grouped controls.
    #[prop(optional)]
    grouped_align: Option<GroupedAlign>,

    /// Allow the grouped controls to span multiple lines.
    #[prop(optional, into)]
    multiline: Signal<bool>,

    /// Make this a horizontal field.
    #[prop(optional, into)]
    horizontal: Signal<bool>,

    /// Optional test attribute (renders as data-* attribute) on the root <div>.
    ///
    /// When provided as a &str or String, this becomes `data-testid="value"`.
    /// You can also pass a full `TestAttr` to override the attribute key.
    #[prop(optional, into)]
    test_attr: Option<TestAttr>,

    /// Child content: typically one or more <Control> blocks.
    children: Children,
) -> impl IntoView {
    let addons_align = addons_align;
    let grouped_align = grouped_align;

    // Build main field class string.
    let class = move || {
        let mut parts = vec!["field".to_string()];

        let extra = classes.get();
        if !extra.trim().is_empty() {
            parts.push(extra);
        }
        if icons_left.get() {
            parts.push("has-icons-left".to_string());
        }
        if icons_right.get() {
            parts.push("has-icons-right".to_string());
        }
        if addons.get() {
            parts.push("has-addons".to_string());
        }
        if let Some(align) = addons_align {
            parts.push(align.bulma().to_string());
        }
        if grouped.get() {
            parts.push("is-grouped".to_string());
        }
        if let Some(align) = grouped_align {
            parts.push(align.bulma().to_string());
        }
        if multiline.get() {
            parts.push("is-grouped-multiline".to_string());
        }

        parts.join(" ")
    };

    // Build optional label node.
    let label_node = {
        let label = label.clone();
        let label_classes = label_classes.clone();
        let horizontal = horizontal.clone();

        move || {
            label.as_ref().map(|text_signal| {
                let text = text_signal.get();
                let mut lc = label_classes.get();

                if lc.trim().is_empty() {
                    if horizontal.get() {
                        view! { <div class="field-label"><label class="label">{text.clone()}</label></div> }
                            .into_any()
                    } else {
                        view! { <label class="label">{text.clone()}</label> }.into_any()
                    }
                } else {
                    if horizontal.get() {
                        // For horizontal, label container is wrapped with "field-label"
                        lc = if lc.trim().is_empty() {
                            "field-label".to_string()
                        } else {
                            format!("{lc} {}", "field-label")
                        };
                        view! {
                            <div class=lc>
                                <label class="label">{text.clone()}</label>
                            </div>
                        }
                        .into_any()
                    } else {
                        // For non-horizontal, append "label" to label classes
                        let final_classes = if lc.trim().is_empty() {
                            "label".to_string()
                        } else {
                            format!("{lc} {}", "label")
                        };
                        view! { <label class=final_classes>{text.clone()}</label> }.into_any()
                    }
                }
            })
        }
    };

    // Build optional help node.
    let help_node = {
        let help = help.clone();
        let help_classes = help_classes.clone();
        let help_has_error = help_has_error.clone();

        move || {
            help.as_ref().map(|help_signal| {
                let mut class_parts = vec!["help".to_string()];
                let extra = help_classes.get();
                if !extra.trim().is_empty() {
                    class_parts.push(extra);
                }
                if help_has_error.get() {
                    class_parts.push("is-danger".to_string());
                }
                let cls = class_parts.join(" ");
                view! { <p class=cls>{help_signal.get()}</p> }.into_view()
            })
        }
    };

    // Body section.
    let body = {
        let horizontal = horizontal.clone();
        move || {
            if horizontal.get() {
                view! { <div class="field-body">{children()}</div> }.into_any()
            } else {
                view! { <>{children()}</> }.into_any()
            }
        }
    };

    let (data_testid, data_cy) = match &test_attr {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    view! {
        <div
            class=class
            attr:data-testid=move || data_testid.clone()
            attr:data-cy=move || data_cy.clone()
        >
            {label_node()}
            {body()}
            {help_node()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::prelude::Control;
    use leptos::prelude::RenderHtml;

    #[test]
    fn field_renders_default_class_and_children() {
        let html = view! { <Field>"X"</Field> }.to_html();
        assert!(
            html.contains(r#"class="field""#),
            "expected base 'field' class, got: {}",
            html
        );
        assert!(html.contains('X'));
    }

    #[test]
    fn field_with_extra_classes() {
        let html = view! { <Field classes="my extra">"Y"</Field> }.to_html();
        assert!(
            html.contains(r#"class="field my extra""#),
            "expected combined classes, got: {}",
            html
        );
        assert!(html.contains('Y'));
    }

    #[test]
    fn field_renders_label_non_horizontal() {
        let html = view! { <Field label="Label">"C"</Field> }.to_html();
        assert!(
            html.contains(r#"class="label""#) && html.contains(">Label<"),
            "expected label element with text, got: {}",
            html
        );
    }

    #[test]
    fn field_help_with_error_flag() {
        let html = view! { <Field help="Oops" help_has_error=true>"C"</Field> }.to_html();
        assert!(
            html.contains(r#"class="help is-danger""#)
                || html.contains(r#"class="help is-danger "#),
            "expected help with is-danger class, got: {}",
            html
        );
        assert!(
            html.contains(">Oops<"),
            "expected help text present, got: {}",
            html
        );
    }

    #[test]
    fn field_horizontal_wraps_label_and_body() {
        let html = view! {
            <Field label="L" horizontal=true>
                <Control><input class="input" type="text"/></Control>
            </Field>
        }
        .to_html();
        assert!(
            html.contains("field-label"),
            "expected horizontal label container 'field-label', got: {}",
            html
        );
        assert!(
            html.contains("field-body"),
            "expected 'field-body' wrapper, got: {}",
            html
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use crate::form::prelude::Control;
    use crate::util::TestAttr;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn field_renders_test_attr_as_data_testid() {
        let html = view! {
            <Field test_attr=TestAttr::test_id("field-test")>
                <Control><input class="input" type="text"/></Control>
            </Field>
        }
        .to_html();

        assert!(
            html.contains(r#"data-testid="field-test""#),
            "expected data-testid attribute; got: {}",
            html
        );
    }

    #[wasm_bindgen_test]
    fn field_no_test_attr_when_not_provided() {
        let html = view! {
            <Field>
                <Control><input class="input" type="text"/></Control>
            </Field>
        }
        .to_html();

        assert!(
            !html.contains("data-testid") && !html.contains("data-cy"),
            "expected no data attribute; got: {}",
            html
        );
    }
}
