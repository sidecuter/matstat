#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl From<(f64, f64)> for Point {
    fn from(data: (f64, f64)) -> Self {
        Self {
            x: data.0,
            y: data.1,
        }
    }
}

pub type Func = Vec<Point>;

pub type FuncSystem = Vec<Func>;
