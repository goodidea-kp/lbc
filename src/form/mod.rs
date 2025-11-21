/*!
Form components module

This directory will host reusable form-related UI components (e.g., Field, Control, Label, Help, Input, Textarea, Select, Checkbox, Radio, etc.).

Note:
- Keep components small and focused.
- Follow naming conventions from src/elements and src/layout.
- Add tests in each component module mirroring existing patterns.
*/

pub mod autocomplete;
pub mod checkbox;
pub mod control;
pub mod field;
pub mod file;
pub mod input;
pub mod radio;
pub mod select;
pub mod textarea;

// Re-export common items here as they are implemented.
pub mod prelude {
    pub use super::autocomplete::AutoComplete;
    pub use super::checkbox::Checkbox;
    pub use super::control::Control;
    pub use super::field::{AddonsAlign, Field, GroupedAlign, LabelSize};
    pub use super::file::File;
    pub use super::input::{Input, InputType};
    pub use super::radio::Radio;
    pub use super::select::{MultiSelect, Select};
    pub use super::textarea::TextArea;
    // pub use super::label::Label;
    // pub use super::help::Help;
}
