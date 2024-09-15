use regex::Regex;
use core::f64;
use std::collections::{HashMap, HashSet};
use super::structs::{FunctionData, Table, TableData};

pub fn function_data(data: &[TableData]) -> Vec<FunctionData> {
    let mut sum = 0;
    let mut prev = None;
    let mut result = Vec::new();
    for td in data {
        result.push(FunctionData::new().set_borders(&(prev, Some(td.x)).into()).set_value(sum));
        sum += td.m;
        prev = Some(td.x);
    }
    result.push(FunctionData::new().set_borders(&(prev, None).into()).set_value(sum));
    result
}

pub fn f_star(datas: &[FunctionData], n: i64) -> Vec<Vec<(f64, f64)>> {
    datas.iter().map(|fd| {
        let mut left = fd.borders.left.unwrap_or(f64::NAN);
        let mut right = fd.borders.right.unwrap_or(f64::NAN);
        let value = fd.value as f64 / n as f64;
        if left.is_nan() {
            left = right-3.0;
        }
        if right.is_nan() {
            right = left + 3.0;
        }
        vec![
            (left, value),
            (right, value),
        ]
    }).collect()
}

pub fn parse_data(values: &str) -> Result<Table, Box<dyn std::error::Error>> {
    let re = Regex::new(r"[; ]+").unwrap();
    let splitted_valued: Vec<&str> = re.split(values).collect();
    // let splited_values = values.split(";");
    // let mut values = Vec::new();
    // for value in splited_values {
    //     values.push(value.parse::<i32>()?);
    // }
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
    Ok((result, n).into())
}
