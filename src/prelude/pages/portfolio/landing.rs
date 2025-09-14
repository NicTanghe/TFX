#![allow(unused_imports)]
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes, A},
    hooks::{use_location, use_params_map},
    path, StaticSegment, WildcardSegment,
};

use crate::prelude::pages::portfolio::assets::{assets_2d_lander, assets_3d_lander};
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
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

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
                <A href=move || format!("/portfolio/{}/2d", id()) exact=true>
                    "2d"
                </A>
                <A href=move || format!("/portfolio/{}/3d", id())>"3d"</A>
            </div>

            <Outlet />
        </div>
    }
}

#[component]
pub fn PortfolioTab() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    let tab = move || params.read().get("tab").unwrap_or_default();

    view! {
        {move || match (id().as_str(), tab().as_str()) {
            ("Assets", "2d") => assets_2d_lander().into_any(),
            ("Assets", "3d") => assets_3d_lander().into_any(),
            ("Compositing", "2d") => view! { <div>"Compositing 2D placeholder"</div> }.into_any(),
            ("Compositing", "3d") => view! { <div>"Compositing 3D placeholder"</div> }.into_any(),
            ("Gamedev", "2d") => view! { <div>"Gamedev 2D placeholder"</div> }.into_any(),
            ("Gamedev", "3d") => view! { <div>"Gamedev 3D placeholder"</div> }.into_any(),
            ("Pipeline", "2d") => view! { <div>"Pipeline 2D placeholder"</div> }.into_any(),
            ("Pipeline", "3d") => view! { <div>"Pipeline 3D placeholder"</div> }.into_any(),
            _ => {

                view! { <p>"No content found."</p> }
                    .into_any()
            }
        }}
    }
}
