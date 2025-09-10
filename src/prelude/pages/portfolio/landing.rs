#![allow(unused_imports)]
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes, A},
    hooks::{use_location, use_params_map},
    path, StaticSegment, WildcardSegment,
};

#[component]
pub fn PortfolioLander() -> impl IntoView {
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

#[component]
pub fn PortfolioInfo() -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "Assets" => "Assets",
        "Compositing" => "Compositing",
        "Gamedev" => "Gamedev",
        _ => "Category not found.",
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>
                    "Contact Info"
                </A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacts/:id route
            <Outlet />
        </div>
    }
}
