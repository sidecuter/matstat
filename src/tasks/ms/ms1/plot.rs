use charming::{
    component::{Axis, DataZoom, DataZoomType, FilterMode, Title}, datatype::CompositeValue, element::{AxisType, LineStyle}, series::Line, Chart, EchartsError
};
use leptos::*;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::JSON::{parse, stringify};
use super::data_processing::f_star;
use super::structs::FunctionData;
use super::themes::Theme;

#[component]
pub fn Plot<'a>(
    data: ReadSignal<Vec<FunctionData>>,
    n: ReadSignal<i64>,
    chart_name: &'a str,
    width: u32,
    height: u32,
    #[prop(default = Theme::Default)]
    theme: Theme
) -> impl IntoView {
    let (id, _) = create_signal(format!("chart_{}", chart_name));
    let theme = theme;
    let _ = create_resource(data, move|data| async move {
        let chart = get_chart(&data, n.get_untracked());
        render(width, height, &id.get_untracked(), &chart, theme).unwrap();
    });

    view! { <div class="container mx-auto w-fit" id=id></div> } 
}

fn get_chart(data: &[FunctionData], n: i64) -> Chart {
    let fd = f_star(data, n);
    let min_x = fd.first().expect("Whoops").first().expect("Whoops").0;
    let max_x = fd.last().expect("Whoops").last().expect("Whoops").0;

    let mut chart = Chart::new()
        .title(Title::new().text("F*(x)"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .min(min_x)
                .max(max_x)
                .split_number(20)
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .max(1.02)
        )
        .data_zoom(
            DataZoom::new()
                .show(true)
                .type_(DataZoomType::Inside)
                .filter_mode(FilterMode::None)
                .x_axis_index(0)
                .start_value(min_x)
                .end_value(max_x)
        )
        .data_zoom(
            DataZoom::new()
                .show(true)
                .type_(DataZoomType::Inside)
                .filter_mode(FilterMode::None)
                .y_axis_index(0)
                .start_value(0)
                .end_value(1.02)
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

pub fn render(width: u32, height: u32, id: &str, chart: &Chart, theme: Theme) -> Result<Echarts, EchartsError> {
    let window = web_sys::window().ok_or(EchartsError::WasmError(
        "no `window` object found".to_string(),
    ))?;
    let document = window.document().ok_or(EchartsError::WasmError(
        "no `document` object found".to_string(),
    ))?;
    let element = document
        .get_element_by_id(id)
        .ok_or(EchartsError::WasmError(format!(
            "no element with id `{}` found",
            id
        )))?;
    let echarts = init(
        &element,
        theme.to_str(),
        to_value(&ChartSize { width, height })
        .unwrap(),
    );
    let json_string = Into::<String>::into(
            stringify(&to_value(chart).unwrap()).unwrap()
        )
        .replace(
            "\"xAxis\":[{",
            "\"xAxis\":[{\"minorTick\":{\"show\":true},\"minorSplitLine\":{\"show\":true},"
        )
        .replace(
            "\"yAxis\":[{",
            "\"yAxis\":[{\"minorTick\":{\"show\":true},\"minorSplitLine\":{\"show\":true},"
        );
    let value = parse(&json_string).unwrap();
    echarts.set_option(value);
    Ok(echarts)
}

#[derive(Serialize)]
struct ChartSize {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = echarts)]
    pub type Echarts;

    #[wasm_bindgen(js_namespace = echarts, js_name = init)]
    fn init(id: &web_sys::Element, theme: &str, size: JsValue) -> Echarts;

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_option(this: &Echarts, option: JsValue);

    #[wasm_bindgen(method, js_name = "resize")]
    pub fn resize(this: &Echarts, opts: JsValue);
}
