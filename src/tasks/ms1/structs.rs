use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone)]
pub struct TableData {
    pub x: i32,
    pub m: i32
}

#[derive(Debug, Default, Clone)]
pub struct Table {
    pub data: Vec<TableData>,
    pub n: i32
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Borders {
    pub start: Option<i32>,
    pub end: Option<i32>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FunctionData {
    pub borders: Borders,
    pub value: i32
}

impl Borders {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_end(&mut self, end: i32) -> Self {
        self.end = Some(end);
        self.clone()
    }


    pub fn set_start(&mut self, start: i32) -> Self {
        self.start = Some(start);
        self.clone()
    }
}

impl From<(Option<i32>, Option<i32>)> for Borders {
    fn from(value: (Option<i32>, Option<i32>)) -> Self {
        Self {
            start: value.0,
            end: value.1
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

    pub fn set_value(&mut self, value: i32) -> Self {
        self.value = value;
        self.clone()
    }
}

impl TableData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_x(&mut self, x: i32) -> Self {
        self.x = x;
        self.clone()
    }

    pub fn set_m(&mut self, m: i32) -> Self {
        self.m = m;
        self.clone()
    }
}

impl From<(i32, i32)> for TableData {
    fn from(item: (i32, i32)) -> Self {
        Self::new().set_x(item.0).set_m(item.1)
    }
}

impl Table {
    pub fn new() -> Self {
        Table::default()
    }

    pub fn set_data(&mut self, td: &[TableData]) -> Self {
        self.data = td.to_vec();
        self.clone()
    }

    pub fn set_n(&mut self, n: i32) -> Self {
        self.n = n;
        self.clone()
    }
}

impl From<(Vec<TableData>, i32)> for Table {
    fn from(item: (Vec<TableData>, i32)) -> Self {
        Table::new().set_data(&item.0).set_n(item.1)
    }
}
