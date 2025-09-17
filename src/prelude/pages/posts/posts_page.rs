#![allow(unused_imports)]
use crate::prelude::pages::posts::{
    posts_logic::{
        get_posts_from_api,
        update_post_api,
    },
    posts_types::{
        PostData,UpdatePostReq
    },
    highlight::{
        extract_code_blocks_from_html,
        highlight_code_blocks,
        assemble_highlighted_content,
        AllStat
    }
};
use crate::prelude::auth_fc::auth_ta::ActiveUser;

use leptos::{
    component,view, IntoView,
    context::use_context,
    prelude::{
        Action, GetUntracked, Effect,AnyView,Set,ReadSignal,WriteSignal,signal,Signal, ElementChild, ClassAttribute, InnerHtmlAttribute, OnAttribute, IntoAny, LocalResource, Resource, Get, CollectView,event_target_value,PropAttribute
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
        <div class="separator-large"></div>
        <div class="blog-posts">
            <h3>"Blog Posts"</h3>

            <div class="blog-intro">
                <p>
                    "Here's some random musings, project updates, experiments that worked "
                    "(and some that didn't), and the occasional rant about technology. "
                    "Nothing too formal — just a place to keep track of ideas in motion."
                </p>
            </div>

            <Outlet />
        </div>
        <div class="separator-small"></div>
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

    // just clone Strings, no Arc needed
    let title = post.title.clone();
    let tags = post.tags.clone();
    let html = post.html.clone();

    let (current_html, set_current_html) = signal(html.clone());
    let original_markdown = post.markdown.clone();

    let (is_editing, set_is_editing) = signal(false);
    let (edit_content, set_edit_content) = signal(original_markdown.clone());

    // ✅ get user context for JWT
    let user = use_context::<ReadSignal<ActiveUser>>().unwrap();

    // 🔑 async local resource: recompute preview whenever edit_content changes
    let preview_html = LocalResource::new(move || {
        let content_string = edit_content.get();
        async move {
            let html_code = markdown::to_html(&content_string);
            let (code_blocks, omark) = extract_code_blocks_from_html(&html_code);
            let syndicated_blocks = highlight_code_blocks(code_blocks).await;
            let tempstat = AllStat {
                orig: omark,
                code: syndicated_blocks,
            };
            assemble_highlighted_content(tempstat).await
        }
    });

    // Save action
    let update_action = {
        let title = title.clone();
        let tags = tags.clone();
        let preview_html = preview_html.clone();
        let user = user.clone();

        Action::new(move |new_markdown: &String| {
            let id = pid;
            let markdown = new_markdown.clone();
            let title = title.clone();
            let tags = tags.clone();
            let html = preview_html.get(); // Option<String>
            let jwt = user.get().token.clone(); // ✅ JWT from context

            async move {
                let req = UpdatePostReq {
                    title: Some(title.clone()),
                    tags: Some(tags.clone()),
                    markdown: Some(markdown),
                    html,
                };
                match update_post_api(id, req, jwt).await {
                    Ok(_) => leptos::logging::log!("✅ Post {id} updated successfully"),
                    Err(e) => leptos::logging::log!("❌ Failed to update post {id}: {:?}", e),
                }
            }
        })
    };

    // ✅ return the UI
    view! {
        <li class="post">
            <h3>{title.clone()}</h3>

            {move || {
                if is_editing.get() {
                    view! {
                        <textarea
                            prop:value=edit_content.get()
                            on:input=move |ev| set_edit_content.set(event_target_value(&ev))
                        />
                    }
                        .into_any()
                } else {
                    view! { <div inner_html=current_html.get().unwrap_or_default() /> }.into_any()
                }
            }}

            <div class="post-actions">
                {move || {
                    if is_editing.get() {
                        view! {
                            <>
                                <button on:click=move |_| {
                                    set_is_editing.set(false);
                                    update_action.dispatch(edit_content.get());
                                }>"Save"</button>
                                <button on:click=move |_| {
                                    set_is_editing.set(false)
                                }>"Cancel"</button>
                            </>
                        }
                            .into_any()
                    } else {
                        view! { <button on:click=move |_| set_is_editing.set(true)>"Edit"</button> }
                            .into_any()
                    }
                }}
            </div>
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
                                    // <h4>"Filtered Posts"</h4>
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

