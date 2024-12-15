use leptos::prelude::*;
use leptos_router::*;
use serde::{Serialize,Deserialize};

use markdown::to_html as markdown_to_html;

// use web_sys::wasm_bindgen::JsCast; //is this use parms map ?
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
    

use leptos::prelude::{component, view, IntoView, ReadSignal, WriteSignal, signal};   //Signal was also in there in 0.6

use leptos_router::components::{Outlet, Route, A};






#[component]

// since you are an idiot I`d  also like the buttons position to be remembered so that they don`t
// all jump to left and then stuf disapears on the right but they disapear in the correct place.
// and dont jump to the left.

pub fn PostList(posts: ReadSignal<Vec<Post>>) -> impl IntoView {
     
    let (selected_tags, set_selected_tags) = signal(Some(Vec::<String>::new));


    // Signals to track previous unique tags and disappearing tags
    let (previous_unique_tags, set_previous_unique_tags) = signal(Some(Vec::<String>::new));
    let (disappearing_tags, set_disappearing_tags) = signal(Some(Vec::<String>::new));

    // Closure to compute current unique and disappearing tags
    let compute_tags = move || {
        let selected = selected_tags.get().as_ref().unwrap_or(&Vec::new).clone();
        let mut current_unique = std::collections::BTreeSet::new;

        for post in posts.get().iter() {
            if selected.is_empty() || selected.iter().all(|tag| post.tags.contains(tag)) {
                current_unique.extend(post.tags.iter().cloned());
            }
        }

        // Get the current unique tags as a Vec<String>
        let current_unique: Vec<String> = current_unique.into_iter().collect();

        // Bind `previous` to avoid dropping temporary Vec
        let previous_tags = previous_unique_tags.get();
        let lifebringer = Vec::new;
        let previous = previous_tags.as_ref().unwrap_or(&lifebringer);
        
        let disappearing = previous
            .iter()
            .filter(|tag| !current_unique.contains(tag))
            .cloned()
            .collect::<Vec<String>>();

        // Update the signals
        set_previous_unique_tags.set(Some(current_unique.clone()));
        set_disappearing_tags.set(Some(disappearing));

        current_unique
    };

    // Render view
    view! {
        <div class="tag-buttons">
            {
                move || {
                    let unique = compute_tags(); // Update unique and disappearing tags
                    let disappearing_tags_list = disappearing_tags.get();
                    
                    let binding = Vec::new;
                    let disappearing = disappearing_tags_list.as_ref().unwrap_or(&binding);

                    // Render unique tags as buttons
                    let unique_buttons = unique.iter().map(|tag| {
                        let tag_clone = tag.clone();
                        let is_selected = selected_tags.get().as_ref()
                            .map_or(false, |tags| tags.contains(&tag_clone));

                        view! {
                            <button
                                class=if is_selected { "tags selected" } else { "tags not-selected" }
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
                    });

                    // Render disappearing tags as buttons with shrinking animation
                    let disappearing_buttons = disappearing.iter().map(|tag| {
                        let tag_clone = tag.clone();

                        view! {
                            <button
                                class="tags disappearing"
                                on:click=move |_| {
                                    log::info!("Disappearing tag clicked: {:?}", tag_clone);
                                }
                            >
                                {tag.clone()}
                            </button>
                        }
                    });

                    // Combine unique and disappearing buttons for display
                    unique_buttons.chain(disappearing_buttons).collect_view()
                }
            }
        </div>




        <div class="post-list-posts">
        // The first entry with a custom class logic
            <A  href="/blog/newpost">"+"</A> // not a tittle this is just the first entry
            {
                move || posts.get().into_iter()
                    .filter(|post| {
                        if let Some(tags) = selected_tags.get().as_ref() {
                            tags.is_empty() || tags.iter().all(|tag| post.tags.contains(tag))
                        } else {
                            true // If no tags are selected, show all posts
                        }
                    })
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
    let id = Memo::new(move |_| params.with(|params| params.get("id").cloned().unwrap_or_default()));

    // Reactively find the post with the matching title (id)
    let post_info = move || {
        let lowercase_id = id().to_lowercase();
        posts.get().iter()
            .find(|post| post.title.to_lowercase() == lowercase_id)
            .cloned()
    };

    view! {
        //<div class= "small_void_void"></div>
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
        <div class="big_void"></div>
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


//ok so the thing with the stuf and so onn.
//it needs 2 things. 1 it needs an input that says what post go`s on top.
//and it also needs an input that setshard tags you cant undo.

// maybe a switch for horizontal / vertical "carousell" (just 2 classes

#[component]
pub fn PostListAll(posts: ReadSignal<Vec<Post>>) -> impl IntoView {
    let (selected_tags, set_selected_tags) = signal(Some(Vec::<String>::new));


    // Signals to track previous unique tags and disappearing tags
    let (previous_unique_tags, set_previous_unique_tags) = signal(Some(Vec::<String>::new));
    let (disappearing_tags, set_disappearing_tags) = signal(Some(Vec::<String>::new));

    // Closure to compute current unique and disappearing tags
    let compute_tags = move || {
        let selected = selected_tags.get().as_ref().unwrap_or(&Vec::new).clone();
        let mut current_unique = std::collections::BTreeSet::new;

        for post in posts.get().iter() {
            if selected.is_empty() || selected.iter().all(|tag| post.tags.contains(tag)) {
                current_unique.extend(post.tags.iter().cloned());
            }
        }

        // Get the current unique tags as a Vec<String>
        let current_unique: Vec<String> = current_unique.into_iter().collect();

        // Bind `previous` to avoid dropping temporary Vec
        let previous_tags = previous_unique_tags.get();
        let lifebringer = Vec::new;
        let previous = previous_tags.as_ref().unwrap_or(&lifebringer);
        
        let disappearing = previous
            .iter()
            .filter(|tag| !current_unique.contains(tag))
            .cloned()
            .collect::<Vec<String>>();

        // Update the signals
        set_previous_unique_tags.set(Some(current_unique.clone()));
        set_disappearing_tags.set(Some(disappearing));

        current_unique
    };

    view! {
        <div class="tag-buttons">
            {
                move || {
                    let unique = compute_tags(); // Update unique and disappearing tags
                    let disappearing_tags_list = disappearing_tags.get();
                    
                    let binding = Vec::new;
                    let disappearing = disappearing_tags_list.as_ref().unwrap_or(&binding);

                    // Render unique tags as buttons
                    let unique_buttons = unique.iter().map(|tag| {
                        let tag_clone = tag.clone();
                        let is_selected = selected_tags.get().as_ref()
                            .map_or(false, |tags| tags.contains(&tag_clone));

                        view! {
                            <button
                                class=if is_selected { "tags selected" } else { "tags not-selected" }
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
                    });

                    // Render disappearing tags as buttons with shrinking animation
                    let disappearing_buttons = disappearing.iter().map(|tag| {
                        let tag_clone = tag.clone();

                        view! {
                            <button
                                class="tags disappearing"
                                on:click=move |_| {
                                    log::info!("Disappearing tag clicked: {:?}", tag_clone);
                                }
                            >
                                {tag.clone()}
                            </button>
                        }
                    });

                    // Combine unique and disappearing buttons for display
                    unique_buttons.chain(disappearing_buttons).collect_view()
                }
            }
        </div>


            // im going to live

        <div class="post-list-cards">
            {
                move || posts.get().into_iter()
                    .filter(|post| {
                        if let Some(tags) = selected_tags.get().as_ref() {
                            tags.is_empty() || tags.iter().all(|tag| post.tags.contains(tag))
                        } else {
                            true // If no tags are selected, show all posts
                        }
                    })
                    .map(|post| {
                        view! {
                            <div class= "skrijver_out decorated">
                                <div class= "text_section">
                                    <div class="horcard">
                                        <div class="card-header">
                                            <h2 class="card-title">{&post.title}</h2>
                                        </div>  
                                        <div class="card-body">
                                            <div inner_html={markdown_to_html(&post.markdown)}></div>
                                        </div>
                                        <div class="card-footer">
                                            <div class="tags">
                                                {
                                                    post.tags.iter().map(|tag| {
                                                        view! {
                                                            <span class="tag">{tag.clone()}</span>
                                                        }
                                                    }).collect_view()
                                                }
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    })
                    .collect_view()
            }
        </div>
    }
}
