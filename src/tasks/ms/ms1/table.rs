use crate::models::table::Table;
use leptos::*;

#[component]
pub fn SequenceTable(data: ReadSignal<Table>, n: ReadSignal<i64>) -> impl IntoView {
    let (headers, _) = create_signal(vec![
        (
            view! {
                <math>
                    <msub>
                        <mi>"x"</mi>
                        <mi>"i"</mi>
                    </msub>
                </math>
            },
            0,
        ),
        (
            view! {
                <math>
                    <msub>
                        <mi>"m"</mi>
                        <mi>"i"</mi>
                    </msub>
                </math>
            },
            1,
        ),
        (
            view! {
                <math>
                    <mfrac>
                        <msub>
                            <mi>"m"</mi>
                            <mi>"i"</mi>
                        </msub>
                        <mi>"n"</mi>
                    </mfrac>
                </math>
            },
            2,
        ),
    ]);
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
                    {move || data().iter().map(|td| {
                        view! { <td class="border">{td.x}</td> }
                    }).collect_view()}
                </tr>
                <tr>
                    {move || data().iter().map(|td| {
                        view! { <td class="border">{td.m}</td> }
                    }).collect_view()}
                </tr>
                <tr>
                    {move || data().iter().map(move |td| {
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
                    }).collect_view()}
                </tr>
            </table>
        </div>
    }
}
