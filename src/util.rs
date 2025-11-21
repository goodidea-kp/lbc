#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl Size {
    pub fn bulma(self) -> &'static str {
        match self {
            Size::Small => "is-small",
            Size::Normal => "",
            Size::Medium => "is-medium",
            Size::Large => "is-large",
        }
    }
}
