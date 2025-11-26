use leptos::prelude::CustomAttribute;

/// Shared size enum used across multiple components.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    /// Large size, maps to Bulma class `is-large`.
    Large,
    /// Medium size, maps to Bulma class `is-medium`.
    Medium,
    /// Normal/default size, maps to an empty class (no size modifier).
    Normal,
    /// Small size, maps to Bulma class `is-small`.
    Small,
}

impl Size {
    /// Returns the Bulma CSS class for this size.
    ///
    /// - `Size::Small` => `"is-small"`
    /// - `Size::Normal` => `""` (no class)
    /// - `Size::Medium` => `"is-medium"`
    /// - `Size::Large` => `"is-large"`
    pub fn bulma(self) -> &'static str {
        match self {
            Size::Small => "is-small",
            Size::Normal => "",
            Size::Medium => "is-medium",
            Size::Large => "is-large",
        }
    }
}

/// A flexible test attribute descriptor used by all components.
///
/// By default, components will render `data-testid="..."`, but callers can
/// override the attribute key if they need a different name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestAttr {
    pub key: String,
    pub value: String,
}

impl TestAttr {
    /// Creates a new `TestAttr` with the given key and value.
    pub fn new<K: Into<String>, V: Into<String>>(key: K, value: V) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Convenience constructor for the common `data-testid` attribute.
    pub fn test_id<V: Into<String>>(value: V) -> Self {
        Self {
            key: "data-testid".to_string(),
            value: value.into(),
        }
    }
}

impl From<String> for TestAttr {
    fn from(value: String) -> Self {
        TestAttr::test_id(value)
    }
}

impl From<&str> for TestAttr {
    fn from(value: &str) -> Self {
        TestAttr::test_id(value.to_string())
    }
}

/// Helper to apply an optional `TestAttr` to a Leptos element.
///
/// Usage in `view!`:
/// ```ignore
/// <div attr:..=test_attr_attr(test_attr)>
///   ...
/// </div>
/// ```
///
/// When `test_attr` is:
/// - `None`  => no attribute is rendered
/// - `Some(TestAttr { key, value })` => renders `key="value"`.
pub fn test_attr_attr(test_attr: Option<TestAttr>) -> impl CustomAttribute<String, String> {
    move |el| {
        if let Some(attr) = &test_attr {
            el.attr(&attr.key, &attr.value)
        } else {
            el
        }
    }
}
