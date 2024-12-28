use crate::models::params::Params;
use crate::models::table::Table;
use leptos::*;

#[component]
pub fn Params(td: ReadSignal<Table>) -> impl IntoView {
    let (params, _) = create_signal(Into::<Params>::into(td.get_untracked()));
    view! {
        <div>
            <p>
                <math display="inline">
                    <mover>
                        <mi>"x"</mi>
                        <mo>"_"</mo>
                    </mover>
                    <mo>"="</mo>
                    {move || generate_x_avg_sintax(td)}
                    <mn>{move || params().x_avg}</mn>
                </math>
            </p>
            <p>
                <math display="inline">
                    <msub>
                        <mi>"D"</mi>
                        <mi>"в"</mi>
                    </msub>
                    <mo>"="</mo>
                    {move || generate_d_v_sintax(td, params.get_untracked().x_avg)}
                    <mn>{move || params().d_v}</mn>
                </math>
            </p>
            <p>
                <math display="inline">
                    <msup>
                        <mi>"S"</mi>
                        <mn>2</mn>
                    </msup>
                    <mo>"="</mo>
                    {move || generate_s_2_sintax(td, params.get_untracked().d_v)}
                    <mn>{move || params().s_2}</mn>
                </math>
            </p>
            <p>
                <math display="inline">
                    <msub>
                        <mi>"σ"</mi>
                        <mo>"в"</mo>
                    </msub>
                    <mo>"="</mo>
                    {move || generate_sigma_v_sintax(params.get_untracked().d_v)}
                    <mn>{move || params().sigma_v}</mn>
                </math>
            </p>
            <p>
                <math display="inline">
                    <mi>"s"</mi>
                    <mo>"="</mo>
                    {move || generate_s_sintax(params.get_untracked().s_2)}
                    <mn>{move || params().s}</mn>
                </math>
            </p>
        </div>
    }
}

fn generate_x_avg_sintax(td: ReadSignal<Table>) -> impl IntoView {
    let n: i64 = td.get_untracked().iter().map(|td| td.m).sum();
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
    let n: i64 = td.get_untracked().iter().map(|td| td.m).sum();
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
                <mn>{td.x}</mn>
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
    let n: i64 = td.get_untracked().iter().map(|td| td.m).sum();
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
            <mn>{move || n - 1}</mn>
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
