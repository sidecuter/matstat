use self::data_processing::parse_data;
use self::formula::Formula;
use self::table::SequenceTable;
use crate::components::plot::Plot;
use crate::models::table::TableData;
use leptos::*;

pub mod data_processing;
pub mod formula;
pub mod table;

#[component]
pub fn Ms1() -> impl IntoView {
    let (data, set_data) = create_signal(Vec::<TableData>::new());
    let (n, set_n) = create_signal(0);
    let (checked, set_checked) = create_signal(false);
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_data_process_click = move |_| {
        let data_string = input_element().expect("Is someone home?").value();
        if let Ok((table, n)) = parse_data(&data_string) {
            set_data(table);
            set_n(n);
            set_checked(true);
        } else {
            set_checked(false);
        }
    };
    view! {
        <div class="text-center mt-2">
            <div class="w-fit mx-auto space-x-1 mb-2">
                <input
                    type="text"
                    size="40"
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-md focus:ring-blue-500 focus:border-blue-500"
                    value="".to_string()
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
                <SequenceTable data=data n=n />
                <Formula data=data n=n />
                <Plot data=data n=n chart_name="f_x" width=800 height=700 />
            </Show>
        </div>
    }
}
