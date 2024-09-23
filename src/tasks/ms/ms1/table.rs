use leptos::*;
use leptos::math::Math;
use crate::models::table::Table;

#[component]
pub fn DataTable(
    headers: ReadSignal<Vec<(HtmlElement<Math>, i32)>>,
    data: ReadSignal<Table>,
    n: ReadSignal<i64>
) -> impl IntoView {
    view! {
        <div class="mx-auto w-fit h-fit flex flex-row overflow-scroll mb-3">
            <table class="border border-collapse">
                <For
                    each=headers
                    key=|(_, id)| *id
                    children=move |(header, _)| {
                        view! {
                            <tr>
                                <td class="font-bold border">{header}</td>
                            </tr>
                        }
                    }
                />
            </table>
            <table class="border border-collapse border-slate-950">
                <tr>
                    <For
                        each=data
                        key=|data| data.x.to_string()
                        children=move |td| {
                            view! { <td class="border">{td.x}</td> }
                        }
                    />
                </tr>
                <tr>
                    <For
                        each=data
                        key=|data| data.x.to_string()
                        children=move |td| {
                            view! { <td class="border">{td.m}</td> }
                        }
                    />
                </tr>
                <tr>
                    <For
                        each=data
                        key=|data| data.x.to_string()
                        children=move |td| {
                            view! {
                                <td class="border">
                                    {
                                        view! {
                                            <math>
                                                <mfrac>
                                                    <mn>{td.m}</mn>
                                                    <mn>{n}</mn>
                                                </mfrac>
                                            </math>
                                        }
                                    }
                                </td>
                            }
                        }
                    />
                </tr>
            </table>
        </div>
    }
}
