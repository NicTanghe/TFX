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
    // Signal to store the currently selected tag for filtering
    let (selected_tag, set_selected_tag) = create_signal(None::<String>);

    // Collect unique tags once, outside the `view!` macro
    let unique_tags = {
        // Create a HashSet to store unique tags
        let mut tags = std::collections::HashSet::new();
        for post in posts.get().iter() {
            tags.extend(post.tags.iter().cloned());
        }
        // Convert HashSet into a Vec for sorted collection of tags
        tags.into_iter().collect::<Vec<String>>()
    };

    //this is absolutly great. it whould probably be less complicated and better ux if it just
    //shows all the tags as buttons that you can toggle to filter. safe this somewhere though as you wil need it

    println!("{:?}", unique_tags); // Debug: Check contents of `unique_tags`

    view! {
        <div class="big_void"></div>
        <div class="post-list">
            <h3>"Posts"</h3>
            
            // Tag filter section
            <div class="tag-filter">
                <label>"Filter by Tag: "</label>
                <select on:input=move |event| {
                    // Safely cast the event target to `HtmlSelectElement` and get its value
                    if let Some(target) = event.target() {
                        if let Ok(select_element) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                            let selected = select_element.value();
                            set_selected_tag.set(if selected.is_empty() { None } else { Some(selected) });
                        }
                    }
                }> // ok look at being able to the list open and selecting multiple entries instead
                   // of using buttons. although a scrollable list of toggle buttons is probably what you
                   // want.
                    <option value="">"All"</option>
                    {
                        // Generate <option> elements for each unique tag
                        unique_tags.iter()
                            .map(|tag| view! {
                                <option value={tag.clone()}>{tag}</option>
                            })
                            .collect_view()
                    }
                </select>
            </div>

            // Posts list
            <div class="post-list-posts">
                {
                    move || posts.get().into_iter()
                        .filter(|post| selected_tag.get().as_deref().map_or(true, |tag| post.tags.contains(&tag.to_string())))
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


