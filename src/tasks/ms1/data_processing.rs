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
        let start;
        let end;
        if let Some(st) = fd.borders.start {
            start = st;
        } else {
            start = 0;
        }
        if let Some(e) = fd.borders.end {
            end = e;
        } else {
            end = start + 3;
        }
        let value = if fd.value == 0 {
            0.
        } else {
            (fd.value as f32) / (n as f32)
        };
        (start..=end).map(move |x| (x as f32, value)).collect()
    }).collect()
}

pub fn parse_data(values: &str) -> Result<Table, Box<dyn std::error::Error>> {
    let splited_values = values.split(";");
    let mut values = Vec::new();
    for value in splited_values {
        values.push(value.parse::<i32>()?);
    }
    let n = values.iter().count() as i32;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut set: HashSet<i32> = HashSet::new();
    for value in values {
        if map.contains_key(&value) {
            if let Some(m) = map.get_mut(&value) {
                *m += 1;
            }
        } else {
            map.insert(value, 1);
        }
        set.insert(value);
    }
    let mut result: Vec<TableData> = map.into_iter().map(|(x, m)| (x, m).into()).collect();
    result.sort_by(|left, right| left.x.cmp(&right.x));
    Ok((result, n).into())
}
