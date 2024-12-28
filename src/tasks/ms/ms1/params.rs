use crate::models::table::Table;
use crate::tasks::ms::ms1::data_processing::{count_d_v, count_s, count_s_2, count_sigma_v, count_x_avg};
use leptos::*;

#[component]
pub fn Params(data: ReadSignal<Table>) -> impl IntoView {
    let n = move || data().iter().map(|td| td.m).sum::<i64>();
    let x_avg = move || count_x_avg(&data(), n());
    let d_v = move || count_d_v(&data(), x_avg(), n());
    let s_2 = move || count_s_2(d_v(), n());
    let sigma_v = move || count_sigma_v(d_v());
    let s = move || count_s(s_2());
    view! {
        <div>
            <p>
                <math display="inline">
                    <mover>
                        <mi>"x"</mi>
                        <mo>"_"</mo>
                    </mover>
                    <mo>"="</mo>
                    {move || generate_x_avg_sintax(data)}
                    <mn>{x_avg}</mn>
                </math>
            </p>
            <p>
                <math display="inline">
                    <msub>
                        <mi>"D"</mi>
                        <mi>"в"</mi>
                    </msub>
                    <mo>"="</mo>
                    {move || generate_d_v_sintax(data, x_avg())}
                    <mn>{d_v}</mn>
                </math>
            </p>
            <p>
                <math display="inline">
                    <msup>
                        <mi>"S"</mi>
                        <mn>2</mn>
                    </msup>
                    <mo>"="</mo>
                    {move || generate_s_2_sintax(data, d_v())}
                    <mn>{s_2}</mn>
                </math>
            </p>
            <p>
                <math display="inline">
                    <msub>
                        <mi>"σ"</mi>
                        <mo>"в"</mo>
                    </msub>
                    <mo>"="</mo>
                    {move || generate_sigma_v_sintax(d_v())}
                    <mn>{sigma_v}</mn>
                </math>
            </p>
            <p>
                <math display="inline">
                    <mi>"s"</mi>
                    <mo>"="</mo>
                    {move || generate_s_sintax(s_2())}
                    <mn>{s}</mn>
                </math>
            </p>
        </div>
    }
}

fn generate_x_avg_sintax(td: ReadSignal<Table>) -> impl IntoView {
    let n = move || td().iter().map(|td| td.m).sum::<i64>();
    view! {
        <mfrac>
            <mi>"1"</mi>
            <mi>"n"</mi>
        </mfrac>
        <mo>"∑"</mo>
        <msub>
            <mi>"x"</mi>
            <mi>"i"</mi>
        </msub>
        <msub>
            <mi>"n"</mi>
            <mi>"i"</mi>
        </msub>
        <mn>"="</mn>
        <mfrac>
            <mn>1</mn>
            <mn>{move || n}</mn>
        </mfrac>
        <mo fence="true" form="prefix">"("</mo>
        <mn>{move || td().first().expect("Should be").x}</mn>
        <mo>*</mo>
        <mn>{move || td().first().expect("Should be").m}</mn>
        {move || td().iter().skip(1).map(|td| 
            view! {
                <mo>+</mo>
                <mn>{td.x}</mn>
                <mo>*</mo>
                <mn>{td.m}</mn>
            }
        ).collect_view()}
        <mo fence="true" form="postfix">")"</mo>
        <mo>"="</mo>
    }
}

fn generate_d_v_sintax(td: ReadSignal<Table>, x_avg: f64) -> impl IntoView {
    let n = move || td().iter().map(|td| td.m).sum::<i64>();
    view! {
        <mfrac>
            <mi>"1"</mi>
            <mi>"n"</mi>
        </mfrac>
        <mo>"∑"</mo>
        <msubsup>
            <mi>"x"</mi>
            <mi>"i"</mi>
            <mn>2</mn>
        </msubsup>
        <msub>
            <mi>"n"</mi>
            <mi>"i"</mi>
        </msub>
        <mo>"-"</mo>
        <msup>
            <mover>
                <mi>"x"</mi>
                <mo>"_"</mo>
            </mover>
            <mn>2</mn>
        </msup>
        <mn>"="</mn>
        <mfrac>
            <mn>1</mn>
            <mn>{move || n}</mn>
        </mfrac>
        <mo fence="true" form="prefix">"("</mo>
        <msup>
            <mn>{move || td().first().expect("Should be").x}</mn>
            <mn>2</mn>
        </msup>
        <mo>*</mo>
        <mn>{move || td().first().expect("Should be").m}</mn>
        {move || td().iter().skip(1).map(|td| 
            view! {
                <mo>+</mo>
                <msup>
                    <mn>{td.x}</mn>
                    <mn>2</mn>
                </msup>
                <mo>*</mo>
                <mn>{td.m}</mn>
            }
        ).collect_view()}
        <mo fence="true" form="postfix">")"</mo>
        <mo>"-"</mo>
        <msup>
            <mn>{move || x_avg}</mn>
            <mn>2</mn>
        </msup>
        <mo>"="</mo>
    }
}

fn generate_s_2_sintax(td: ReadSignal<Table>, d_v: f64) -> impl IntoView {
    let n = move || td().iter().map(|td| td.m).sum::<i64>();
    view! {
        <mfrac>
            <mi>"n"</mi>
            <mi>"n-1"</mi>
        </mfrac>
        <msub>
            <mi>"D"</mi>
            <mi>"в"</mi>
        </msub>
        <mn>"="</mn>
        <mfrac>
            <mn>{move || n}</mn>
            <mn>{move || n() - 1}</mn>
        </mfrac>
        <mn>{move || d_v}</mn>
        <mo>"="</mo>
    }
}

fn generate_sigma_v_sintax(d_v: f64) -> impl IntoView {
    view! {
        <msqrt>
            <msub>
                <mi>"D"</mi>
                <mi>"в"</mi>
            </msub>
        </msqrt>
        <mn>"="</mn>
        <msqrt>
            <mn>{move || d_v}</mn>
        </msqrt>
        <mo>"="</mo>
    }
}

fn generate_s_sintax(s_2: f64) -> impl IntoView {
    view! {
        <msqrt>
            <msup>
                <mi>"S"</mi>
                <mn>2</mn>
            </msup>
        </msqrt>
        <mn>"="</mn>
        <msqrt>
            <mn>{move || s_2}</mn>
        </msqrt>
        <mo>"="</mo>
    }
}
