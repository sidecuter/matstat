use leptos::*;
use leptos_router::*;

pub mod tasks;
pub mod models;
pub mod components;

use crate::tasks::ms::ms1::Ms1;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen min-w-full justify-items-center justify-center">
            <Router>
                <Routes>
                    <Route path="/" view=Ms1 />
                    <Route
                        path="/ms2"
                        view=|| {
                            view! { <p>"Yo"</p> }
                        }
                    />
                </Routes>
            </Router>
        </div>
    }
}
