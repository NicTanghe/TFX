#![allow(unused_imports)]
use crate::prelude::pages::posts::{
    posts_logic::{
        get_posts_from_api,
        update_post_api,
    },
    posts_types::{
        PostData,UpdatePostReq
    }
};

use leptos::{
    component,view, IntoView,
    prelude::{
        Action, GetUntracked, Effect,AnyView,Set,ReadSignal,WriteSignal,signal,Signal, ElementChild, ClassAttribute, InnerHtmlAttribute, OnAttribute, IntoAny, Resource, Get, CollectView,event_target_value,PropAttribute
        },
    suspense::Suspense,
    logging::log,
    *
    }; // for #[server] and ServerFnError


use leptos_router::components::{
    A,Outlet
    };

use markdown;
use std::sync::Arc;
//use serde::{Deserialize, Serialize};
//use std::collections::BTreeSet;

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

fn render_unique_button(
    tag: String,
    selected_tags: ReadSignal<Vec<String>>,
    set_selected_tags: WriteSignal<Vec<String>>,
) -> AnyView {
    let is_selected = selected_tags.get().contains(&tag);
    let tag_clone = tag.clone();

    view! {
        <button
            class=if is_selected { "tags selected" } else { "tags not-selected" }
            on:click=move |_| {
                let mut sel = selected_tags.get();
                if sel.contains(&tag_clone) {
                    sel.retain(|t| t != &tag_clone);
                } else {
                    sel.push(tag_clone.clone());
                }
                set_selected_tags.set(sel);
            }
        >
            {tag}
        </button>
    }
    .into_any()
}

/// Disappearing button renderer
fn render_disappearing_button(tag: String) -> AnyView {
    view! { <button class="tags disappearing">{tag}</button> }.into_any()
}


// ========== CHILD: Loads posts + filters by :id ==========

// --- new component for each post row ---

#[component]
pub fn PostRow(post: PostData) -> impl IntoView {
    let pid = post.post_id;

    // Wrap in Arc so we can share safely across closures
    let title: Arc<String> = Arc::new(post.title);
    let tags: Arc<Vec<String>> = Arc::new(post.tags);
    let html = post.html;

    let original_markdown: Arc<String> = Arc::new(post.markdown.clone());

    let (is_editing, set_is_editing) = signal(false);
    let (edit_content, set_edit_content) = signal((*original_markdown).clone());

    // Live preview resource — re-renders whenever edit_content changes
    let preview_html = Signal::derive(move || {
    let md = edit_content.get();
    markdown::to_html(&md)
    });
    // Save action (unchanged)
    let update_action = {
        let title = title.clone();
        let tags = tags.clone();
        let preview_html = preview_html.clone(); // capture it

        Action::new(move |new_markdown: &String| {
            let id = pid;
            let markdown = new_markdown.clone();
            let title = title.clone();
            let tags = tags.clone();
            let html = preview_html.get(); // ✅ latest derived html

            async move {
                let req = UpdatePostReq {
                    title: Some((*title).clone()),
                    tags: Some((*tags).clone()),
                    markdown: Some(markdown),
                    html: Some(html), // ✅ send to server
                };
                match update_post_api(id, req).await {
                    Ok(_) => leptos::logging::log!("✅ Post {id} updated successfully"),
                    Err(e) => leptos::logging::log!("❌ Failed to update post {id}: {:?}", e),
                }
            }
        })
    }; // ✅ close update_action here

    view! {
        <li class="post">
            <h5>{(*title).clone()}</h5>

            <div class="tags">
                {tags.iter().map(|t| view! { <span class="tag">{t.clone()}</span> }).collect_view()}
            </div>

            {move || {
                let original_markdown = original_markdown.clone();
                if is_editing.get() {
                    view! {
                        <div class="edit-block edit-split">
                            <div class="edit-pane">
                                <textarea
                                    class="markdown-editor"
                                    prop:value=edit_content.get()
                                    on:input=move |ev| {
                                        set_edit_content.set(event_target_value(&ev));
                                    }
                                />
                                <div class="edit-actions">
                                    <button
                                        class="save-btn"
                                        type="button"
                                        on:click=move |_| {
                                            update_action.dispatch(edit_content.get());
                                            set_is_editing.set(false);
                                        }
                                    >
                                        "Save"
                                    </button>
                                    <button
                                        class="cancel-btn"
                                        type="button"
                                        on:click=move |_| {
                                            set_edit_content.set((*original_markdown).clone());
                                            set_is_editing.set(false);
                                        }
                                    >
                                        "Cancel"
                                    </button>
                                </div>
                            </div>
                            <div class="preview-pane">
                                <h6 class="preview-title">"Live preview"</h6>
                                {move || {
                                    view! { <div class="html" inner_html=preview_html.get()></div> }
                                }}
                            </div>
                        </div>
                    }
                        .into_any()
                } else if let Some(h) = html.clone() {

                    // --- EDIT MODE: editor (left) + live HTML preview (right) ---
                    // --- VIEW MODE: show saved HTML if present ---
                    view! {
                        <div>
                            <div class="html" inner_html=h></div>
                        </div>
                    }
                        .into_any()
                } else {
                    // --- VIEW MODE fallback: show original markdown ---
                    view! {
                        <div>
                            <pre class="markdown">{(*original_markdown).clone()}</pre>
                        </div>
                    }
                        .into_any()
                }
            }}

            <button class="edit-btn" type="button" on:click=move |_| set_is_editing.set(true)>
                "Edit"
            </button>
        </li>
    }
}
pub fn posts_loader() -> impl IntoView {
    let posts = Resource::new(|| (), |_| async { get_posts_from_api().await });

    // selected tags
    let (selected_tags, set_selected_tags) = signal::<Vec<String>>(vec![]);

    // computed state as signals
    let (unique_tags, set_unique_tags) = signal::<Vec<String>>(vec![]);
    let (prev_tags, set_prev_tags) = signal::<Vec<String>>(vec![]);
    let (disappearing_tags, set_disappearing_tags) = signal::<Vec<String>>(vec![]);

    // derive unique + disappearing tags
    Effect::new(move |_| {
        use std::collections::BTreeSet;

        let selected = selected_tags.get();
        let mut set = BTreeSet::new();

        if let Some(Ok(list)) = posts.get() {
            for post in list.iter() {
                if selected.is_empty() || selected.iter().all(|tag| post.tags.contains(tag)) {
                    set.extend(post.tags.iter().cloned());
                }
            }
        }

        let curr: Vec<String> = set.into_iter().collect();

        let prev = prev_tags.get_untracked();
        let disappearing = prev
            .iter()
            .filter(|t| !curr.contains(*t))
            .cloned()
            .collect::<Vec<String>>();

        set_unique_tags.set(curr.clone());
        set_disappearing_tags.set(disappearing);
        set_prev_tags.set(curr);
    });

    // ✅ return the actual UI
    view! {
        // --- TAG BUTTONS ---
        <Suspense fallback=move || {
            view! {
                <div class="tag-buttons">
                    <p>"Loading tags…"</p>
                </div>
            }
        }>
            <div class="tag-buttons">
                {move || {
                    let unique = unique_tags.get();
                    let disappearing = disappearing_tags.get();
                    let unique_buttons: Vec<AnyView> = unique
                        .into_iter()
                        .map(|tag| render_unique_button(tag, selected_tags, set_selected_tags))
                        .collect();
                    let disappearing_buttons: Vec<AnyView> = disappearing
                        .into_iter()
                        .map(render_disappearing_button)
                        .collect();
                    unique_buttons.into_iter().chain(disappearing_buttons).collect_view()
                }}
            </div>
        </Suspense>

        // --- POSTS LIST ---
        <div class="posts-wrapper">
            <Suspense fallback=move || {
                view! { <p>"Loading posts…"</p> }
            }>
                {move || {
                    match posts.get() {
                        Some(Ok(list)) => {
                            view! {
                                <div>
                                    <h4>"Filtered Posts"</h4>
                                    <ul class="posts">
                                        {list
                                            .into_iter()
                                            .filter(|p| {
                                                let tags = selected_tags.get();
                                                tags.is_empty()
                                                    || tags.iter().all(|tag| p.tags.contains(tag))
                                            })
                                            .map(|p| view! { <PostRow post=p.clone() /> })
                                            .collect_view()}
                                    </ul>
                                </div>
                            }
                                .into_any()
                        }
                        Some(Err(e)) => {

                            view! {
                                <div>
                                    <p class="error">{format!("Failed to load posts: {e}")}</p>
                                </div>
                            }
                                .into_any()
                        }
                        None => {

                            view! {
                                <div>
                                    <p>"Loading posts…"</p>
                                </div>
                            }
                                .into_any()
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}

