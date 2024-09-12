use leptos::*;
use self::formula::Formula;
use self::plot::draw;
use self::data_processing as dp;
use self::structs::{TableData, FunctionData};

pub mod formula;
// pub mod table;
pub mod data_processing;
pub mod plot;
pub mod structs;

#[component]
pub fn Ms1() -> impl IntoView {
    let (value, set_value) = create_signal("".to_string());
    let (data, set_data) = create_signal(vec![TableData::new()]);
    let (n, set_n) = create_signal(0);
    let (checked, set_checked) = create_signal(false);
    let input_element: NodeRef<html::Input> = create_node_ref();
    let (function_data, set_function_data) = create_signal(
        vec![
            FunctionData::new()
        ]
    );
    view! {
        <div class="text-center">
            <div class="w-fit mx-auto">
                <input type="text"
                    value=value
                    node_ref=input_element
                />
                <button
                    on:click=move |_| {
                        let data_string = input_element.get().expect("Is someone home?").value();
                        let parsed_data = dp::parse_data(&data_string);
                        let fd;
                        if let Ok(table) = parsed_data {
                            set_data.set(table.data);
                            fd = dp::function_data(&data.get_untracked());
                            set_n.set(table.n);
                            set_function_data.set(fd);
                        }
                        set_value.set(data_string);
                        set_checked.set(true);
                        let _ = draw("test_chart", &function_data.get_untracked(), n.get_untracked());
                    }
                >"Расчитать функцию"</button>
            </div>
            <Show
                when=move || { checked.get() }
                fallback= || view!{}
            >
                <Formula conditions=function_data n=n/>
                <canvas class="mx-auto" width="1000px" height="800px" id="test_chart"></canvas>
            </Show>
            <div>"Copy this repo and change the chart function in the source code and make different charts"</div>
        </div>
    }
}