use crate::prelude::pages::posts::{
    posts_logic::get_posts_from_api,
    posts_types::PostData,
};

use leptos::{
    component,view, IntoView,
    prelude::{
        ElementChild, ClassAttribute, InnerHtmlAttribute, create_memo, OnAttribute, IntoAny, Resource, Get, CollectView
        },
    suspense::Suspense,
    *
    }; // for #[server] and ServerFnError


use leptos_router::components::{
    A,Outlet
    };
use serde::{Deserialize, Serialize};



#[component]
pub fn PostsLander() -> impl IntoView {
    view! {
        <div class="portfolio-list">
            <h3>"Portfolio"</h3>
            <div class="portfolio-list-categories">
                <A href="Assets">"Assets"</A>
                <A href="Compositing">"Compositing"</A>
                <A href="Gamedev">"Gamedev"</A>
                <A href="pipeline">"pipeline"</A>
            </div>
            <Outlet />
        </div>
    }
}

// ========== CHILD: Loads posts + filters by :id ==========
#[component]
pub fn PostsLoader() -> impl IntoView {
    // --- Resource: fetch posts from API once ---
    let posts = Resource::new(
        || (), 
        |_| async { get_posts_from_api().await } // Result<Vec<Post>, ServerFnError>
    );

    view! {
        <div class="posts-wrapper">
            <Suspense fallback=move || view! { <p>"Loading posts…"</p> }>
                {move || {
                    match posts.get() {
                        // --- Success ---
                        Some(Ok(list)) => view! {
                            <div>
                                <h4>"All Posts"</h4>
                                <ul class="posts">
                                    {list.into_iter().map(|p| view! {
                                        <li class="post">
                                            <h5>{p.title.clone()}</h5>

                                            <div class="tags">
                                                {p.tags.iter().map(|t| 
                                                    view! { <span class="tag">{t.clone()}</span> }
                                                ).collect_view()}
                                            </div>

                                            {move || match p.html.clone() {
                                                Some(html) => view! {
                                                    <div>
                                                        <div class="html" inner_html=html></div>
                                                    </div>
                                                }.into_view(),

                                                None => view! {
                                                    <div>
                                                        <pre class="markdown">{p.markdown.clone()}</pre>
                                                    </div>
                                                }.into_view(),
                                            }}
                                        </li>
                                    }).collect_view()}
                                </ul>
                            </div>
                        },

                        // --- Error ---
                        Some(Err(e)) => view! {
                            <div>
                                <p class="error">{format!("Failed to load posts: {e}")}</p>
                            </div>
                        },

                        // --- Loading ---
                        None => view! {
                            <div>
                                <p>"Loading posts…"</p>
                            </div>
                        },
                    }
                }}
            </Suspense>
        </div>
    }
}
