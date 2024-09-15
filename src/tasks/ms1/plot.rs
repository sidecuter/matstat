use charming::{
    component::{Axis, Title},
    datatype::CompositeValue,
    element::{AxisType, LineStyle},
    series::Line,
    Chart,
    WasmRenderer,
};
use leptos::*;
use super::data_processing::f_star;
use super::structs::FunctionData;

#[component]
pub fn Chart<'a>(
    data: ReadSignal<Vec<FunctionData>>,
    n: ReadSignal<i64>,
    chart_name: &'a str,
    width: u32,
    height: u32,
) -> impl IntoView {
    let (id, _) = create_signal(format!("chart_{}", chart_name));
    let _ = create_local_resource(|| (), move |_| async move {
        let renderer = WasmRenderer::new(width, height);
        let chart = get_chart(&data(), n());
        renderer.render(&id(), &chart).unwrap();
    });

    view! {
        <div class="container mx-auto w-fit" id=id></div>
    } 
}

fn get_chart(data: &[FunctionData], n: i64) -> Chart {
    let fd = f_star(data, n);

    let mut chart = Chart::new()
        .title(Title::new().text("F*(x)"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .split_number(50)
                .max(1.02)
        );
    for data in fd {
        chart = chart.series(
            Line::new().data(
                data.into_iter().map(|(x, y)| Into::<CompositeValue>::into(vec![x, y])).collect()
            ).show_symbol(false)
            .line_style(LineStyle::new().color("#5470c6"))
        );
    }
    chart
}
