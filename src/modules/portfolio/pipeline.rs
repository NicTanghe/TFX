use leptos::*; 
use leptos_router::Outlet;


use crate::modules::blog_posts::blog_compo::Post;
#[component]
pub fn Portf_editing(posts_l1: ReadSignal<Vec<Post>>) -> impl IntoView {
    // Define categories directly as a Vec<String>

    // Insert `category_links` into the final view
    view! {
        <div class="big_void"></div>
        <h1>"Pipeline ! is allowing you to tell stories quickly"</h1>
        <Outlet/>


        //insert carousel of all pipeline blogposts. after some BS

    }
}
