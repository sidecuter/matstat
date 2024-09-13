use leptos::*;

#[component]
pub fn table(
    headers: ReadSignal<Vec<String>>,
    data: ReadSignal<Vec<Vec<String>>>
) -> impl IntoView {
    view! {
        <div>
            // <For
            <table>// each=headers
            // key=|header| header
            // children=move ||
            // />
            </table>
            <table></table>
        </div>
    }
}