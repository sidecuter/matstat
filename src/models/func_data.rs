#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Borders {
    pub left: Option<f64>,
    pub right: Option<f64>
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FormulaData {
    pub borders: Borders,
    pub value: i64
}

impl Borders {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_end(&mut self, right: f64) -> Self {
        self.right = Some(right);
        *self
    }


    pub fn set_start(&mut self, left: f64) -> Self {
        self.left = Some(left);
        *self
    }
}

impl From<(Option<f64>, Option<f64>)> for Borders {
    fn from(value: (Option<f64>, Option<f64>)) -> Self {
        Self {
            left: value.0,
            right: value.1
        }
    }
}

impl FormulaData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_borders(&mut self, borders: &Borders) -> Self {
        self.borders = *borders;
        self.clone()
    }

    pub fn set_value(&mut self, value: i64) -> Self {
        self.value = value;
        self.clone()
    }
}
