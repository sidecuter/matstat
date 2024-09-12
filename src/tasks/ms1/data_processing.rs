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

pub fn f_star(datas: &[FunctionData], n: i32) -> Vec<Vec<(f32, f32)>> {
    datas.iter().map(|fd| {
        let start = fd.borders.start.unwrap_or(0);
        let end = fd.borders.end.unwrap_or(start+3);
        (start..=end).map(move |x| (x as f32, (fd.value as f32) / (n as f32))).collect()
    }).collect()
}

pub fn parse_data(values: &str) -> Result<Table, Box<dyn std::error::Error>> {
    let splited_values = values.split(";");
    let mut values = Vec::new();
    for value in splited_values {
        values.push(value.parse::<i32>()?);
    }
    let n = values.len() as i32;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut set: HashSet<i32> = HashSet::new();
    for value in values {
        *map.entry(value).or_insert(0) += 1;
        set.insert(value);
    }
    let mut result: Vec<TableData> = map.into_iter().map(|(x, m)| (x, m).into()).collect();
    result.sort_by(|left, right| left.x.cmp(&right.x));
    Ok((result, n).into())
}
