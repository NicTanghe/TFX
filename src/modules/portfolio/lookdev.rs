use leptos::prelude::*; 
use leptos_router::components::Outlet;
#[component]
pub fn Portf_lookdev() -> impl IntoView {
    // Define categories directly as a Vec<String>

    // Insert `category_links` into the final view
    view! {
        <div class="big_void"></div>
        <h1>"Lookdev ! is mostly glue and a bit of storytelling"</h1>
        <Outlet/>
    }
}
