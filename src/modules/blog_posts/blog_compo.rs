use leptos::*;
use leptos_router::*;
use serde::{Serialize,Deserialize};

use markdown::to_html as markdown_to_html;

 use web_sys::wasm_bindgen::JsCast;
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
    

use leptos::{component, view, IntoView, ReadSignal, WriteSignal, create_signal, Signal};
use leptos_router::A;







#[component]
pub fn PostList(posts: ReadSignal<Vec<Post>>) -> impl IntoView {
    let (selected_tags, set_selected_tags) = create_signal(Some(Vec::<String>::new()));

    let unique_tags = move ||{
        let mut tags = std::collections::BTreeSet::new();
        for post in posts.get().iter() {
            tags.extend(post.tags.iter().cloned());
        }
        tags.into_iter().collect::<Vec<String>>()
    };

    view! {
        <div class="tag-buttons">
            {
              move ||  unique_tags().iter().map(|tag| {
                    let tag_clone = tag.clone();
                    let is_selected = selected_tags.get().as_ref()
                        .map_or(false, |tags| tags.contains(&tag_clone));

                    view! {
                        <button
                            class=if is_selected { "tags selected" } else { "tags " }
                            on:click=move |_| {
                                let mut selected = selected_tags.get().clone().unwrap_or_default();
                                if selected.contains(&tag_clone) {
                                    selected.retain(|t| t != &tag_clone); 
                                } else {
                                    selected.push(tag_clone.clone()); 
                                }
                                set_selected_tags.set(Some(selected)); 
                            }
                        >
                            {tag.clone()}
                        </button>
                    }
                }).collect_view()
            }
        </div>


        <div class="post-list-posts">
            {
                move || posts.get().into_iter()
                    .filter(|post| {
                        // Ensure that all selected tags are present in post tags
                        if let Some(tags) = selected_tags.get().as_ref() {
                            tags.is_empty() || tags.iter().all(|tag| post.tags.contains(tag))
                        } else {
                            true // If no tags are selected, show all posts
                        }
                    }) //you can also make a switch to an ore filter
                    .map(|post| {
                        let href = format!("/blog/{}", post.title.to_lowercase());
                        view! {
                            <A href={href}>{&post.title}</A>
                        }
                    })
                    .collect_view()
            }
        </div>

        <Outlet/>
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
        <div class= "void_small"></div>
        <div class= "text_section">
            <div class= "skrijver_out decorated">
                <h1 class="blog_title" key={id()}>
                    {
                        move || {
                            match post_info() {
                                Some(post) => format!("{}", post.title),
                                None => "Task not found.".to_string(),
                            }
                        }
                    }
                </h1>
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
            </div>
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
            <Route  path=":id" view=move ||  view! { <PostInfo posts={posts} _set_posts={set_posts}/> }>
                <Route path="" view=move || view! {
                }/>
        </Route>
        </Route>
    }
}


