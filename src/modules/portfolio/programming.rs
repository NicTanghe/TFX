use leptos::*; 
use leptos_router::Outlet;

use crate::modules::blog_posts::blog_compo::{Post,PostListAll};

#[component]
pub fn Portf_programming(posts: ReadSignal<Vec<Post>>) -> impl IntoView {
    // Define categories directly as a Vec<String>

    // Insert `category_links` into the final view
    view! {
        <div class="big_void"></div>
        <h1>"Automate/Streamline everything"</h1>
        <Outlet/>

        <PostListAll posts/>


    }
}
