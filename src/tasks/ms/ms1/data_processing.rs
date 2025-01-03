use crate::models::{
    func_data::FormulaData,
    point_2d::*,
    table::{Table, TableData},
};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn function_data(data: &[TableData]) -> Vec<FormulaData> {
    let mut sum = 0;
    let mut prev = None;
    let mut result = Vec::new();
    for td in data {
        result.push(
            FormulaData::new()
                .set_borders(&(prev, Some(td.x)).into())
                .set_value(sum),
        );
        sum += td.m;
        prev = Some(td.x);
    }
    result.push(
        FormulaData::new()
            .set_borders(&(prev, None).into())
            .set_value(sum),
    );
    result
}

pub fn f_star(datas: &[FormulaData], n: i64) -> FuncSystem {
    let k = (1.0 + (n as f64).log10() * 3.322).round();
    let first = datas
        .first()
        .expect("Expected non empty sequence")
        .borders
        .right
        .expect("Expected existing value");
    let last = datas
        .last()
        .expect("Expected non empty sequence")
        .borders
        .left
        .expect("Expected existing value");
    let h = ((last - first) / k * 100.0).round() / 100.0;
    datas
        .iter()
        .map(|fd| {
            let mut left = fd.borders.left.unwrap_or(f64::NAN);
            let mut right = fd.borders.right.unwrap_or(f64::NAN);
            let value = fd.value as f64 / n as f64;
            if left.is_nan() {
                left = right - h;
            }
            if right.is_nan() {
                right = left + h;
            }
            vec![(left, value).into(), (right, value).into()]
        })
        .collect()
}

pub fn parse_data(values: &str) -> Result<(Table, i64), Box<dyn std::error::Error>> {
    let re = Regex::new(r"[; ]+").unwrap();
    let values = values.replace(",", ".");
    let splitted_valued: Vec<&str> = re.split(&values).collect();
    let n = splitted_valued.len() as i64;
    let mut map: HashMap<&str, i64> = HashMap::new();
    let mut set: HashSet<&str> = HashSet::new();
    for value in splitted_valued {
        *map.entry(value).or_insert(0) += 1;
        set.insert(value);
    }
    let mut result: Vec<TableData> = Vec::new();
    for (x, m) in map {
        let x: f64 = x.parse()?;
        result.push((x, m).into());
    }
    result.sort_by(|left, right| left.x.partial_cmp(&right.x).unwrap());
    Ok((result, n))
}

pub fn count_x_avg(t: &Table, n: i64) -> f64 {
    let sum: f64 = t.iter().map(|td| td.m as f64 * td.x).sum();
    sum / n as f64
}

pub fn count_d_v(t: &Table, x_avg: f64, n: i64) -> f64 {
    let sum: f64 = t.iter().map(|td| td.m as f64 * td.x * td.x).sum();
    sum / n as f64 - x_avg * x_avg
} 

pub fn count_s_2(d_v: f64, n: i64) -> f64 {
    d_v * n as f64 / (n - 1) as f64
}

pub fn count_s(s_2: f64) -> f64 {
    s_2.sqrt()
}

pub fn count_sigma_v(d_v: f64) -> f64 {
    d_v.sqrt()
}
