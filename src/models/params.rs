use crate::tasks::ms::ms1::data_processing::{count_d_v, count_s, count_s_2, count_sigma_v, count_x_avg};

use super::table::Table;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Params {
    pub(crate) x_avg: f64,
    pub(crate) d_v: f64,
    pub(crate) s_2: f64,
    pub(crate) sigma_v: f64,
    pub(crate) s: f64
}

impl Params {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<Table> for Params {
    fn from(value: Table) -> Self {
        let n: i64 = value.iter().map(|t| t.m).sum();
        let x_avg = count_x_avg(&value, n);
        let d_v = count_d_v(&value, x_avg, n);
        let s_2 = count_s_2(d_v, n);
        let sigma_v = count_sigma_v(d_v);
        let s = count_s(s_2);
        Self {x_avg, d_v, s_2, sigma_v, s}
    }
}
