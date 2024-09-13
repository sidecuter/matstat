use charming::{
    component::{Axis, Title},
    datatype::CompositeValue,
    element::{AxisType, LineStyle},
    series::Line,
    Chart,
    WasmRenderer,
};
use super::data_processing::f_star;
use super::structs::FunctionData;

pub fn draw(canvas_id: &str, data: &[FunctionData], n: i32) -> Result<(), Box<dyn std::error::Error>> {
    let renderer = WasmRenderer::new(600, 700);
    let chart = get_chart(data, n);
    renderer.render(canvas_id, &chart).unwrap();
    todo!()
}

fn get_chart(data: &[FunctionData], n: i32) -> Chart {
    let fd = f_star(data, n);
    let split_x = fd.last().expect("Whoops").last().expect("Whoops").0;

    let mut chart = Chart::new()
        .title(Title::new().text("Simple Line Chart"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .split_number(split_x)
                .min(0)
                .max(split_x)
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .split_number(50)
                .min(0)
                .max(1.02)
        );
    for data in fd {
        chart = chart.series(
            Line::new().data(
                data.into_iter().map(|(x, y)| Into::<CompositeValue>::into(vec![x as f64, y as f64])).collect()
            ).show_symbol(false)
            .line_style(LineStyle::new().color("#5470c6"))
        );
    }
    chart
}
