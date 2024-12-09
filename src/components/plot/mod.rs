use crate::models::point_2d::*;
use crate::models::table::Table;
use crate::tasks::ms::ms1::data_processing::{f_star, function_data};
use charming::{
    component::{Axis, DataZoom, DataZoomType, FilterMode, Title},
    datatype::CompositeValue,
    element::{AxisType, LineStyle},
    series::Line,
    Chart, EchartsError,
};
use js_sys::JSON::parse;
use leptos::*;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use themes::Theme;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub mod themes;

#[component]
#[allow(clippy::needless_lifetimes)]
pub fn Plot<'a>(
    data: ReadSignal<Table>,
    n: ReadSignal<i64>,
    chart_name: &'a str,
    width: u32,
    height: u32,
    #[prop(default = Theme::Default)] theme: Theme,
    #[prop(default = create_signal(
        vec![
            (
                "\"xAxis\":[{".to_string(),
                "\"xAxis\":[{\"minorTick\":{\"show\":true},\"minorSplitLine\":{\"show\":true},".to_string()
            ),
            (
                "\"yAxis\":[{".to_string(),
                "\"yAxis\":[{\"minorTick\":{\"show\":true},\"minorSplitLine\":{\"show\":true},".to_string()
            )
        ]
    ).0)]
    replacers: ReadSignal<Vec<(String, String)>>,
) -> impl IntoView {
    let (id, _) = create_signal(format!("chart_{}", chart_name));
    let _ = create_resource(data, move |data| async move {
        let func_data = f_star(&function_data(&data), n());
        let chart = get_chart(&func_data);
        render(
            &replacers.get_untracked(),
            width,
            height,
            &id.get_untracked(),
            &chart,
            theme,
        )
        .unwrap();
    });

    view! { <div class="container mx-auto w-fit" id=id></div> }
}

fn get_chart(fd: &[Func]) -> Chart {
    let min_x = fd.first().expect("Whoops").first().expect("Whoops").x;
    let max_x = fd.last().expect("Whoops").last().expect("Whoops").x;

    let mut chart = Chart::new()
        .title(Title::new().text("F*(x)"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .min(min_x)
                .max(max_x)
                .split_number(20),
        )
        .y_axis(Axis::new().type_(AxisType::Value).max(1.02))
        .data_zoom(
            DataZoom::new()
                .show(true)
                .type_(DataZoomType::Inside)
                .filter_mode(FilterMode::None)
                .x_axis_index(0)
                .start_value(min_x)
                .end_value(max_x),
        )
        .data_zoom(
            DataZoom::new()
                .show(true)
                .type_(DataZoomType::Inside)
                .filter_mode(FilterMode::None)
                .y_axis_index(0)
                .start_value(0)
                .end_value(1.02),
        );
    for data in fd {
        let points: Vec<CompositeValue> = data
            .iter()
            .map(|&p| Into::<CompositeValue>::into(vec![p.x, p.y]))
            .collect();
        chart = chart.series(
            Line::new()
                .data(points)
                .show_symbol(false)
                .line_style(LineStyle::new().color("#5470c6")),
        );
    }
    chart
}

pub fn render(
    replacers: &[(String, String)],
    width: u32,
    height: u32,
    id: &str,
    chart: &Chart,
    theme: Theme,
) -> Result<Echarts, EchartsError> {
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
        to_value(&ChartSize { width, height }).unwrap(),
    );
    let mut json_string = serde_json::to_string(&chart).unwrap();
    for (old, new) in replacers {
        json_string = json_string.replace(old, new);
    }
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
