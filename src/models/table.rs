#[derive(Debug, Default, Clone)]
pub struct TableData {
    pub x: f64,
    pub m: i64
}

pub type Table = Vec<TableData>;

impl TableData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_x(&mut self, x: f64) -> Self {
        self.x = x;
        self.clone()
    }

    pub fn set_m(&mut self, m: i64) -> Self {
        self.m = m;
        self.clone()
    }
}

impl From<(f64, i64)> for TableData {
    fn from(item: (f64, i64)) -> Self {
        Self::new().set_x(item.0).set_m(item.1)
    }
}
