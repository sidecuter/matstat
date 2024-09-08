use leptos::*;
use leptos_router::*;

pub mod tasks;

use crate::tasks::ms1::Ms1;

#[component]
pub fn App() -> impl IntoView {
    view! {
        // <Router>
        //     <div class="min-h-screen min-w-full justify-items-center justify-center">
        //         <Routes>
        //             <Route path="/" view=Ms1 />
        //         </Routes>
        //     </div>
        // </Router>
        <div class="min-h-screen min-w-full justify-items-center justify-center">
            <Ms1/>
        </div>
    }
}
