//! Utility types shared across multiple components.
//!
//! Currently provides a `Size` enum and helpers to map it to Bulma CSS classes.

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
