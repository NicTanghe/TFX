use leptos::*;
use leptos_router::*;
use serde::{Serialize,Deserialize};

use markdown::to_html as markdown_to_html;

//use crate::modules::blog_posts::blog_fn::*; 

//use leptos_markdown::*;

// Post struct from your API
#[derive(PartialEq, Clone,Serialize, Deserialize, Debug)] // Ensure Post is Cloneable
pub struct Post {
    #[serde(rename = "id")]
    pub post_id: i32,
    pub title: String,
    pub markdown: String,
    pub tags: Vec<String>,
}   
    

#[component]
pub fn PostList(posts: ReadSignal<Vec<Post>>) -> impl IntoView {
    view! {
        <div class="post-list">
            <h3>"Posts"</h3>
            <div class="post-list-posts">
                {move || posts.get().into_iter().map(|post| {
                    // Use the title from the Post struct for href and display
                    let href = format!("/blog/{}", post.title.to_lowercase());
                    view! {
                        <A href={href}>{&post.title}</A>
                    }
                }).collect_view()}
            </div>
            <Outlet/>
        </div>
    }
}



#[component]
pub fn PostInfo(posts: ReadSignal<Vec<Post>>, _set_posts: WriteSignal<Vec<Post>>) -> impl IntoView {
    let params = use_params_map();
    let id = create_memo(move |_| params.with(|params| params.get("id").cloned().unwrap_or_default()));

    // Reactively find the post with the matching title (id)
    let post_info = move || {
        let lowercase_id = id().to_lowercase();
        posts.get().iter()
            .find(|post| post.title.to_lowercase() == lowercase_id)
            .cloned()
    };

    view! {
        <h4 key={id()}>
            {
                move || {
                    match post_info() {
                        Some(post) => format!("TASK: {}", post.title),
                        None => "Task not found.".to_string(),
                    }
                }
            }
        </h4>
        <div key={id()}>
            {
                move || {
                    match post_info() {
                        Some(post) => view! {
                            <div inner_html={markdown_to_html(&post.markdown)}></div>  // Render Markdown as raw HTML
                        },
                        None => view! {
                            <div inner_html={markdown_to_html("### something went wrong")}></div>
                        },
                    }
                }
            }
        </div>
        <Outlet/>
    }
}


pub fn post_routes(
    posts: ReadSignal<Vec<Post>>, 
    set_posts: WriteSignal<Vec<Post>>
) -> impl IntoView {
    view! {
        <Route path="/blog" view=move || view! { <PostList posts={posts} /> }>   
            <Route path="" view=|| view! {
                <p>"Select a post to view more info."</p>
            }/>
            <Route path=":id" view=move ||  view! { <PostInfo posts={posts} _set_posts={set_posts}/> }>
                <Route path="" view=move || view! {
                <div class="tab">"Post Info"</div>
                }/>
        </Route>
        </Route>
    }
}


