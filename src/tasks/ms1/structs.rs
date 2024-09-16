#[derive(Debug, Default, Clone)]
pub struct TableData {
    pub x: f64,
    pub m: i64
}

pub type Table = Vec<TableData>;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Borders {
    pub left: Option<f64>,
    pub right: Option<f64>
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FunctionData {
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

impl FunctionData {
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

// impl Table {
//     pub fn new() -> Self {
//         Table::default()
//     }

//     pub fn set_data(&mut self, td: &[TableData]) -> Self {
//         self.data = td.to_vec();
//         self.clone()
//     }

//     pub fn set_n(vec![
//         FunctionData::new()
//     ]&mut self, n: i64) -> Self {
//         self.n = n;
//         self.clone()
//     }
// }

// impl From<(Vec<TableData>, i64)> for Table {
//     fn from(item: (Vec<TableData>, i64)) -> Self {
//         Table::new().set_data(&item.0).set_n(item.1)
//     }
// }
