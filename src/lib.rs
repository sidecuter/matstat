use leptos::*;
//use leptos_router::*;

pub mod tasks;
// pub mod components;

use crate::tasks::ms1::Ms1;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen min-w-full justify-items-center justify-center">
            <Ms1 />
        </div>
    }
}
