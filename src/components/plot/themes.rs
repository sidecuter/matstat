#[derive(Clone, Copy)]
pub enum Theme {
    Default,
    Dark,
}

impl Theme {
    pub fn to_str(&self) -> &'static str {
        match self {
            Theme::Default => "",
            Theme::Dark => "dark",
        }
    }
}
