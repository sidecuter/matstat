use data_processing::f_star;
use leptos::*;
use self::formula::Formula;
use self::table::DataTable;
use self::data_processing as dp;
use crate::components::plot::Plot;
use crate::models::{table::TableData, func_data::FormulaData, point_2d::Func};

pub mod formula;
pub mod table;
pub mod data_processing;

#[component]
pub fn Ms1() -> impl IntoView {
    let (value, _) = create_signal("".to_string());
    let (data, set_data) = create_signal(Vec::<TableData>::new());
    let (n, set_n) = create_signal(0);
    let (checked, set_checked) = create_signal(false);
    let (headers, _) = create_signal(
        vec![
            (view! {
                <math>
                    <msub>
                        <mi>"x"</mi>
                        <mi>"i"</mi>
                    </msub>
                </math>
            }, 0),
            (view! {
                <math>
                    <msub>
                        <mi>"m"</mi>
                        <mi>"i"</mi>
                    </msub>
                </math>
            }, 1),
            (view! {
                <math>
                    <mfrac>
                        <msub>
                            <mi>"m"</mi>
                            <mi>"i"</mi>
                        </msub>
                        <mi>"n"</mi>
                    </mfrac>
                </math>
            },2)
        ]
    );
    let (formula_data, set_formula_data) = create_signal(
        Vec::<FormulaData>::new()
    );
    let (function_data, set_function_data) = create_signal(
        Vec::<Func>::new()
    );
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_data_process_click = move |_| {
        let data_string = input_element().expect("Is someone home?").value();
        let parsed_data = dp::parse_data(&data_string);
        let fd;
        if let Ok((table, n)) = parsed_data {
            fd = dp::function_data(&table);
            set_data(table);
            set_n(n);
            set_function_data(f_star(&fd, n));
            set_formula_data(fd);
        }
        set_checked(true);
    };
    view! {
        <div class="text-center mt-2">
            <div class="w-fit mx-auto space-x-1 mb-2">
                <input
                    type="text"
                    size="40"
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-md focus:ring-blue-500 focus:border-blue-500"
                    value=value
                    node_ref=input_element
                />
                <button
                    class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-md text-sm px-5 py-2.5 text-center"
                    on:click=on_data_process_click
                >
                    "Расчитать функцию"
                </button>
            </div>
            <Show when=checked fallback=|| view! {}>
                <DataTable headers=headers data=data n=n />
                <Formula conditions=formula_data n=n />
                <Plot data=function_data chart_name="f_x".into() width=800 height=700 />
            </Show>
        </div>
    }
}
