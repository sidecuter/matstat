use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use super::data_processing::f_star;
use super::structs::FunctionData;

pub fn draw(canvas_id: &str, data: &[FunctionData], n: i32) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 15.0).into();
    root.fill(&WHITE)?;
    let result = f_star(data, n);
    let last = result.last().expect("Whoops").last().expect("Whoops").0;
    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption("f*(x)", font)
        .x_label_area_size(20u32)
        .y_label_area_size(20u32)
        .build_cartesian_2d(0f32..last, 0f32..1f32)?;
    chart.configure_mesh().x_labels(last as usize + 1).y_labels(11).draw()?;
    for line in result {
        chart.draw_series(LineSeries::new(line.into_iter(),&RED,))?;
    }
    root.present()?;
    Ok(())
}
