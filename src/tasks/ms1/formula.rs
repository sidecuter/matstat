use leptos::*;
use super::structs::FunctionData;


#[component]
pub fn Formula(
    conditions: ReadSignal<Vec<FunctionData>>,
    n: ReadSignal<i32>
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
                    {move || {
                        conditions()
                            .into_iter()
                            .map(|fd| {
                                view! {
                                    <mtr>
                                        <mtd>
                                            {move || match fd.value {
                                                0 => view! { <mn>"0"</mn> }.into_any(),
                                                _ if fd.value / n() == 1 => {
                                                    view! { <mn>"1"</mn> }.into_any()
                                                }
                                                _ => {
                                                    view! {
                                                        <mfrac>
                                                            <mn>{move || fd.value}</mn>
                                                            <mn>{n}</mn>
                                                        </mfrac>
                                                    }
                                                        .into_any()
                                                }
                                            }} <mtext>", если"</mtext> <mspace width="5px" />
                                            {move || {
                                                fd.borders
                                                    .start
                                                    .is_some()
                                                    .then(|| {
                                                        view! {
                                                            <mn>{move || fd.borders.start.unwrap().to_string()}</mn>
                                                            <mo>"<"</mo>
                                                        }
                                                    })
                                            }} <mi>"x"</mi>
                                            {move || {
                                                fd.borders
                                                    .end
                                                    .is_some()
                                                    .then(|| {
                                                        view! {
                                                            <mo>"≤"</mo>
                                                            <mn>{move || fd.borders.end.unwrap().to_string()}</mn>
                                                        }
                                                    })
                                            }}
                                        </mtd>
                                    </mtr>
                                }
                            })
                            .collect_view()
                    }}
                </mtable>
            </mrow>
        </math>
    }
}
