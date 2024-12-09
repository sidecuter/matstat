use leptos::*;
use crate::models::func_data::FormulaData;
use crate::models::table::Table;
use crate::tasks::ms::ms1::data_processing::function_data;

#[component]
pub fn Formula(
    data: ReadSignal<Table>,
    n: ReadSignal<i64>
) -> impl IntoView {
    view! {
        <math>
            <msup>
                <mi>"F"</mi>
                <mi>"*"</mi>
            </msup>
            <mo fence="true" form="prefix" stretchy="false">
                "("
            </mo>
            <mi>"x"</mi>
            <mo fence="true" form="postfix" stretchy="false">
                ")"
            </mo>
            <mo>"="</mo>
            <mrow>
                <mo fence="true" form="prefix" stretchy="true">
                    "["
                </mo>
                <mtable>
                    {move || function_data(&data())
                        .into_iter()
                        .map(|fd| {
                            formula_row(fd, n)
                        })
                        .collect_view()
                    }
                </mtable>
            </mrow>
        </math>
    }
}

fn formula_row(
    fd: FormulaData,
    n: ReadSignal<i64>
) -> impl IntoView {
    view! {
        <mtr>
            <mtd>
                {first_row_element(fd, n)}
                <mtext>", если"</mtext>
                <mspace width="5px" />
                {left_row_element(fd)}
                <mi>"x"</mi>
                {right_row_element(fd)}
            </mtd>
        </mtr>
    }
}

fn first_row_element(fd: FormulaData, n: ReadSignal<i64>) -> impl IntoView {
    view! {
        {move || match fd.value {
            0 => view! { <mn>"0"</mn> }.into_any(),
            _ if fd.value / n() == 1 => view! { <mn>"1"</mn> }.into_any(),
            _ => view! {
                    <mfrac>
                        <mn>{move || fd.value}</mn>
                        <mn>{n}</mn>
                    </mfrac>
                }.into_any()
        }}
    }
}

fn left_row_element(fd: FormulaData) -> impl IntoView {
    view! {
        {move || fd.borders
            .left
            .is_some()
            .then(|| {
                view! {
                    <mn>{move || fd.borders.left.unwrap().to_string()}</mn>
                    <mo>"<"</mo>
                }
            })
        }
    }
}

fn right_row_element(fd: FormulaData) -> impl IntoView {
    view! {
        {move || fd.borders
            .right
            .is_some()
            .then(|| {
                view! {
                    <mo>"≤"</mo>
                    <mn>{move || fd.borders.right.unwrap().to_string()}</mn>
                }
            })
        }
    }
}
