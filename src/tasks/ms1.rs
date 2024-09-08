use std::collections::{HashMap, HashSet};
use std::ops::Range;
use leptos::*;
use log::info;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

pub type Data = (Vec<(i32, i32)>, i32);

#[component]
pub fn Ms1() -> impl IntoView {
    let (value, set_value) = create_signal("".to_string());
    let (data, set_data) = create_signal(vec![(0, 0)]);
    let (n, set_n) = create_signal(0i32);
    // let render_resource = create_local_resource(move || (data, n), |(data, n)| async move {
    //     let data: Vec<(i32, i32)> = data.get_untracked();
    //     draw("test_chart", &data, n.get_untracked()).await
    // });

    view! {
        <div class="text-center">
            <div class="w-fit mx-auto">
                <input type="text"
                    on:input=move |ev| {
                        let data_string = event_target_value(&ev);
                        let parsed_data = parse_data(&data_string);
                        if let Ok((parsed_data, parsed_n)) = parsed_data {
                            set_data.set(parsed_data);
                            set_n.set(parsed_n);
                        }
                        set_value.set(data_string);
                    }
                    prop:value=value
                />
                <button
                    on:click=move |_| {
                        draw("test_chart", &(data.get_untracked()), n.get_untracked());
                    }
                >"Расчитать функцию"</button>
            </div>
            <canvas class="mx-auto" width="1000px" height="800px" id="test_chart"></canvas>
            <div>"Copy this repo and change the chart function in the source code and make different charts"</div>
        </div>
    }
}

pub fn draw(canvas_id: &str, data: &[(i32, i32)], n: i32) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 15.0).into();

    root.fill(&WHITE)?;
    let last = data.last().expect("Whoops").0;
    let range = 0..last+3;
    let result = f_star(&data, n, range);

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption("f*(x)", font)
        .x_label_area_size(20u32)
        .y_label_area_size(20u32)
        .build_cartesian_2d(0f32..(last+2) as f32, 0f32..1f32)?;

    chart.configure_mesh().x_labels((last+3) as usize).y_labels(11).draw()?;
    for line in result {
        chart.draw_series(LineSeries::new(
            line.into_iter(),
            &RED,
        ))?;
    }

    root.present()?;
    Ok(())
}

fn f_star(data: &[(i32, i32)], n: i32, range: Range<i32>) -> Vec<Vec<(f32, f32)>> {
    let mut data_iter = data.iter();
    let mut pair = data_iter.next().unwrap();
    let mut num: i32 = pair.0;
    let last = data.last().expect("whoops").0;
    let last_pair: (i32, i32) = (last, n);
    let mut result: Vec<Vec<(f32, f32)>> = Vec::new();
    let mut sum: f32 = 0.;
    let mut temp: Vec<(f32, f32)> = Vec::new();
    for i in range {
        if i >= num && pair.1 < n {
            temp.push((i as f32, sum));
            if sum < 1. {
                sum += pair.1 as f32 / n as f32;
            }
            pair = data_iter.next().unwrap_or(&last_pair);
            num = pair.0;
            result.push(temp);
            temp = Vec::new();
        }
        temp.push((i as f32, sum));
    }
    result.push(temp);
    result
}

fn parse_data(values: &str) -> Result<Data, Box<dyn std::error::Error>> {
    let splited_values = values.split(";");
    let mut values = Vec::new();
    for value in splited_values {
        values.push(value.parse::<i32>()?);
    }
    info!("{:?}", values);
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
    let mut result: Vec<(i32, i32)> = map.into_iter().map(|(x, m)| (x, m)).collect();
    info!("{:?}", result);
    sort(&mut result);
    info!("{:?}", result);
    info!("{}", n);
    Ok((result, n))
}

fn sort(data: &mut [(i32, i32)]) {
    for i in 0..data.len() {
        for j in 0..data.len() - i - 1 {
            if data[j + 1].0 < data[j].0 {
                data.swap(j, j + 1);
            }
        }
    }
}
