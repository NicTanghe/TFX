#![allow(unused_imports)]

use leptos::{
    component,
    logging::log,
    prelude::{
        event_target_value, signal, Action, AnyView, ClassAttribute, CollectView, Effect,
        ElementChild, Get, GetUntracked, InnerHtmlAttribute, IntoAny, LocalResource, OnAttribute,
        PropAttribute, ReadSignal, Resource, Set, Signal, View, WriteSignal,
    },
    suspense::Suspense,
    view, IntoView, *,
}; // for #[server] and ServerFnError

use leptos_router::components::{self, Outlet, A};

#[component]
pub fn assets_2d_lander() -> impl IntoView {
    view! {
        <section class="parallax-section">
            <div class="parallax-bg"></div>
            <div class="parallax-content">
                <h2>"Assets — 2D"</h2>
                <p>"Check out my 2D assets and design work."</p>
                <div class="buttons">
                    <a href="#projects" class="btn">
                        "See Projects"
                    </a>
                    <a href="#contact" class="btn">
                        "Contact Me"
                    </a>
                </div>
            </div>
        </section>

        <section class="normal-section">
            <h3>"2D Projects"</h3>
            <p>"This is where your 2D portfolio content continues…"</p>
        </section>
    }
    .into_view()
}
#[component]
pub fn assets_3d_lander() -> impl IntoView {
    view! {
        <section class="parallax-section">
            <div class="parallax-bg"></div>
            <div class="parallax-content">
                <h2>"Assets — 3D"</h2>
                <p>"Explore my 3D models, renders, and case studies."</p>
                <div class="buttons">
                    <a href="#projects" class="btn">
                        "See Projects"
                    </a>
                    <a href="#contact" class="btn">
                        "Contact Me"
                    </a>
                </div>
            </div>
        </section>

        <section class="normal-section">
            <h3>"3D Projects"</h3>
            <p>"This is where your 3D portfolio content continues…"</p>
        </section>
    }
    .into_view()
}
