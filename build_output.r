   Compiling home_portal v0.1.0 (/home/ghuber/dev/leptos/tfx_portal/pointseven)
error[E0432]: unresolved imports `leptos::ReadSignal`, `leptos::WriteSignal`, `leptos::create_signal`, `leptos::Signal`
  --> src/modules/blog_posts/blog_compo.rs:23:41
   |
23 | use leptos::{component, view, IntoView, ReadSignal, WriteSignal, create_signal, Signal};
   |                                         ^^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^ no `Signal` in the root
   |                                         |           |            |
   |                                         |           |            no `create_signal` in the root
   |                                         |           no `WriteSignal` in the root
   |                                         no `ReadSignal` in the root
   |
   = help: consider importing one of these items instead:
           crate::modules::blog_posts::blog_compo::SignalTypes::ReadSignal
           leptos::prelude::ReadSignal
           leptos::prelude::SignalTypes::ReadSignal
   = help: consider importing this struct instead:
           leptos::prelude::WriteSignal
   = help: consider importing this struct instead:
           leptos::prelude::Signal

error[E0432]: unresolved import `leptos_router::A`
  --> src/modules/blog_posts/blog_compo.rs:24:5
   |
24 | use leptos_router::A;
   |     ^^^^^^^^^^^^^^^^ no `A` in the root
   |
help: consider importing one of these items instead
   |
24 | use leptos::html::A;
   |     ~~~~~~~~~~~~~~~
24 | use leptos::svg::A;
   |     ~~~~~~~~~~~~~~
24 | use tracing_subscriber::fmt::writer::EitherWriter::A;
   |     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0432]: unresolved import `leptos::prelude::logging`
 --> src/modules/blog_posts/blog_comms.rs:5:5
  |
5 |     logging,
  |     ^^^^^^^ no `logging` in `prelude`
  |
  = help: consider importing this module instead:
          leptos::logging

error[E0432]: unresolved import `leptos::prelude::ev`
 --> src/modules/textfields/skrijf.rs:1:23
  |
1 | use leptos::prelude::{ev::SubmitEvent, *};
  |                       ^^ could not find `ev` in `prelude`

error[E0432]: unresolved imports `leptos::prelude::logging`, `leptos::prelude::SignalGet`
 --> src/modules/people/people_comms.rs:3:5
  |
3 |     logging,
  |     ^^^^^^^ no `logging` in `prelude`
...
6 |     SignalGet,
  |     ^^^^^^^^^
  |     |
  |     no `SignalGet` in `prelude`
  |     help: a similar name exists in the module: `Signal`
  |
  = help: consider importing this module instead:
          leptos::logging

error[E0432]: unresolved imports `leptos_router::A`, `leptos_router::Outlet`, `leptos_router::use_navigate`
 --> src/modules/portfolio/home.rs:3:5
  |
3 |     A,Outlet,NavigateOptions,use_navigate,
  |     ^ ^^^^^^                 ^^^^^^^^^^^^
  |     | |                      |
  |     | |                      no `use_navigate` in the root
  |     | |                      help: a similar name exists in the module: `navigate`
  |     | no `Outlet` in the root
  |     no `A` in the root
  |
  = help: consider importing one of these items instead:
          leptos::html::A
          leptos::svg::A
          tracing_subscriber::fmt::writer::EitherWriter::A

error[E0432]: unresolved import `leptos_router::Outlet`
 --> src/modules/portfolio/editing.rs:2:5
  |
2 | use leptos_router::Outlet;
  |     ^^^^^^^^^^^^^^^^^^^^^ no `Outlet` in the root

error[E0432]: unresolved import `leptos_router::Outlet`
 --> src/modules/portfolio/writing.rs:3:5
  |
3 | use leptos_router::Outlet;
  |     ^^^^^^^^^^^^^^^^^^^^^ no `Outlet` in the root

error[E0432]: unresolved import `leptos_router::Outlet`
 --> src/modules/portfolio/pipeline.rs:2:5
  |
2 | use leptos_router::Outlet;
  |     ^^^^^^^^^^^^^^^^^^^^^ no `Outlet` in the root

error[E0432]: unresolved import `leptos_router::Outlet`
 --> src/modules/portfolio/modelling.rs:2:5
  |
2 | use leptos_router::Outlet;
  |     ^^^^^^^^^^^^^^^^^^^^^ no `Outlet` in the root

error[E0432]: unresolved import `leptos_router::Outlet`
 --> src/modules/portfolio/simulations.rs:2:5
  |
2 | use leptos_router::Outlet;
  |     ^^^^^^^^^^^^^^^^^^^^^ no `Outlet` in the root

error[E0432]: unresolved import `leptos_router::Outlet`
 --> src/modules/portfolio/compositing.rs:2:5
  |
2 | use leptos_router::Outlet;
  |     ^^^^^^^^^^^^^^^^^^^^^ no `Outlet` in the root

error[E0432]: unresolved import `leptos_router::Outlet`
 --> src/modules/portfolio/lookdev.rs:2:5
  |
2 | use leptos_router::Outlet;
  |     ^^^^^^^^^^^^^^^^^^^^^ no `Outlet` in the root

error[E0432]: unresolved import `leptos_router::Outlet`
 --> src/modules/portfolio/programming.rs:2:5
  |
2 | use leptos_router::Outlet;
  |     ^^^^^^^^^^^^^^^^^^^^^ no `Outlet` in the root

error[E0432]: unresolved import `tracing`
 --> src/app.rs:6:5
  |
6 | use tracing::debug;
  |     ^^^^^^^ use of undeclared crate or module `tracing`

error[E0433]: failed to resolve: use of undeclared crate or module `logging`
   --> src/modules/textfields/skrijf.rs:237:17
    |
237 |                 logging::log!("Submission timed out - setting status to error");
    |                 ^^^^^^^ use of undeclared crate or module `logging`

error[E0433]: failed to resolve: use of undeclared crate or module `logging`
   --> src/modules/textfields/skrijf.rs:228:9
    |
228 |         logging::log!("Submission started");
    |         ^^^^^^^ use of undeclared crate or module `logging`

error[E0433]: failed to resolve: use of undeclared crate or module `logging`
  --> src/modules/blog_posts/blog_fn.rs:68:17
   |
68 |                 logging::log!("Failed to delete post from API: {}", e);
   |                 ^^^^^^^ use of undeclared crate or module `logging`

error[E0433]: failed to resolve: use of undeclared crate or module `logging`
  --> src/modules/blog_posts/blog_fn.rs:65:17
   |
65 |                 logging::log!("Successfully deleted post from API with id: {}", id);
   |                 ^^^^^^^ use of undeclared crate or module `logging`

error[E0433]: failed to resolve: use of undeclared crate or module `logging`
  --> src/modules/blog_posts/blog_fn.rs:60:9
   |
60 |         logging::log!("After calling delete_post_from_api for id: {}", id);
   |         ^^^^^^^ use of undeclared crate or module `logging`

error[E0433]: failed to resolve: use of undeclared crate or module `logging`
  --> src/modules/blog_posts/blog_fn.rs:58:9
   |
58 |         logging::log!("Before calling delete_post_from_api for id: {}", id);
   |         ^^^^^^^ use of undeclared crate or module `logging`

error[E0433]: failed to resolve: use of undeclared crate or module `logging`
  --> src/modules/blog_posts/blog_fn.rs:33:13
   |
33 |             logging::log!("Error fetching posts: {:?}", err);
   |             ^^^^^^^ use of undeclared crate or module `logging`

error[E0412]: cannot find type `WriteSignal` in this scope
  --> src/app.rs:61:39
   |
61 | fn UncontrolledComponent(set_user_l3: WriteSignal<ActiveUser>) -> impl IntoView {
   |                                       ^^^^^^^^^^^ not found in this scope
   |
help: consider importing one of these structs
   |
1  + use crate::app::reactive::signal::WriteSignal;
   |
1  + use leptos::prelude::WriteSignal;
   |

error[E0425]: cannot find function `create_signal` in this scope
  --> src/app.rs:64:36
   |
64 |     let (userName, set_userName) = create_signal("".to_string());
   |                                    ^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing one of these functions
   |
1  + use crate::app::reactive::signal::create_signal;
   |
1  + use leptos::prelude::create_signal;
   |

error[E0425]: cannot find function `create_signal` in this scope
  --> src/app.rs:65:36
   |
65 |     let (password, set_password) = create_signal("".to_string());
   |                                    ^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing one of these functions
   |
1  + use crate::app::reactive::signal::create_signal;
   |
1  + use leptos::prelude::create_signal;
   |

error[E0412]: cannot find type `NodeRef` in this scope
  --> src/app.rs:68:25
   |
68 |     let username_input: NodeRef<Input> = create_node_ref();
   |                         ^^^^^^^ not found in this scope
   |
help: consider importing one of these structs
   |
1  + use crate::app::tachys::reactive_graph::node_ref::NodeRef;
   |
1  + use leptos::prelude::NodeRef;
   |

error[E0425]: cannot find function `create_node_ref` in this scope
  --> src/app.rs:68:42
   |
68 |     let username_input: NodeRef<Input> = create_node_ref();
   |                                          ^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing one of these functions
   |
1  + use crate::app::tachys::reactive_graph::node_ref::create_node_ref;
   |
1  + use leptos::prelude::create_node_ref;
   |

error[E0412]: cannot find type `NodeRef` in this scope
  --> src/app.rs:69:25
   |
69 |     let password_input: NodeRef<Input> = create_node_ref();
   |                         ^^^^^^^ not found in this scope
   |
help: consider importing one of these structs
   |
1  + use crate::app::tachys::reactive_graph::node_ref::NodeRef;
   |
1  + use leptos::prelude::NodeRef;
   |

error[E0425]: cannot find function `create_node_ref` in this scope
  --> src/app.rs:69:42
   |
69 |     let password_input: NodeRef<Input> = create_node_ref();
   |                                          ^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing one of these functions
   |
1  + use crate::app::tachys::reactive_graph::node_ref::create_node_ref;
   |
1  + use leptos::prelude::create_node_ref;
   |

error[E0425]: cannot find function `spawn_local` in crate `leptos`
  --> src/app.rs:91:25
   |
91 |                 leptos::spawn_local(async move {
   |                         ^^^^^^^^^^^ not found in `leptos`
   |
help: consider importing one of these functions
   |
1  + use crate::app::task::spawn_local;
   |
1  + use leptos::task::spawn_local;
   |
help: if you import `spawn_local`, refer to it directly
   |
91 -                 leptos::spawn_local(async move {
91 +                 spawn_local(async move {
   |

error[E0412]: cannot find type `ReadSignal` in this scope
   --> src/app.rs:172:14
    |
172 |     user_l2: ReadSignal<ActiveUser>, 
    |              ^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these structs
    |
1   + use crate::app::reactive::signal::ReadSignal;
    |
1   + use leptos::prelude::ReadSignal;
    |

error[E0412]: cannot find type `WriteSignal` in this scope
   --> src/app.rs:173:18
    |
173 |     set_user_l2: WriteSignal<ActiveUser>, 
    |                  ^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these structs
    |
1   + use crate::app::reactive::signal::WriteSignal;
    |
1   + use leptos::prelude::WriteSignal;
    |

error[E0412]: cannot find type `ReadSignal` in this scope
   --> src/app.rs:174:18
    |
174 |     is_hiding_l: ReadSignal<u8>
    |                  ^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these structs
    |
1   + use crate::app::reactive::signal::ReadSignal;
    |
1   + use leptos::prelude::ReadSignal;
    |

error[E0412]: cannot find type `ReadSignal` in this scope
   --> src/app.rs:275:20
    |
275 | fn NavBar(user_l1: ReadSignal<ActiveUser>,set_user_l1: WriteSignal<ActiveUser>) -> impl IntoView {
    |                    ^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these structs
    |
1   + use crate::app::reactive::signal::ReadSignal;
    |
1   + use leptos::prelude::ReadSignal;
    |

error[E0412]: cannot find type `WriteSignal` in this scope
   --> src/app.rs:275:56
    |
275 | fn NavBar(user_l1: ReadSignal<ActiveUser>,set_user_l1: WriteSignal<ActiveUser>) -> impl IntoView {
    |                                                        ^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these structs
    |
1   + use crate::app::reactive::signal::WriteSignal;
    |
1   + use leptos::prelude::WriteSignal;
    |

error[E0425]: cannot find function `use_location` in this scope
   --> src/app.rs:277:20
    |
277 |     let location = use_location();
    |                    ^^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::hooks::use_location;
    |
1   + use leptos_router::hooks::use_location;
    |

error[E0425]: cannot find function `create_signal` in this scope
   --> src/app.rs:279:38
    |
279 |     let (show_card, set_show_card) = create_signal(false); //proly dont need
    |                                      ^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::reactive::signal::create_signal;
    |
1   + use leptos::prelude::create_signal;
    |

error[E0425]: cannot find function `create_signal` in this scope
   --> src/app.rs:282:34
    |
282 |     let (is_hiding, set_hiding) =create_signal(2 as u8);
    |                                  ^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::reactive::signal::create_signal;
    |
1   + use leptos::prelude::create_signal;
    |

error[E0425]: cannot find value `A` in this scope
   --> src/app.rs:294:22
    |
294 |                     <A  // note right now the light turns of and we want that but its not on purpase its a bug so fix
    |                      ^ not found in this scope
    |
help: consider importing one of these items
    |
1   + use crate::app::components::A;
    |
1   + use crate::app::either::EitherOf10::A;
    |
1   + use crate::app::either::EitherOf11::A;
    |
1   + use crate::app::either::EitherOf12::A;
    |
      and 17 other candidates

error[E0425]: cannot find value `A` in this scope
   --> src/app.rs:299:22
    |
299 | ...   <A class=move || format!("navlink{}", if is_active("/contacts") { " nb-active" } else { "" }) href="/contacts">"Contacts"</A>
    |        ^ not found in this scope
    |
help: consider importing one of these items
    |
1   + use crate::app::components::A;
    |
1   + use crate::app::either::EitherOf10::A;
    |
1   + use crate::app::either::EitherOf11::A;
    |
1   + use crate::app::either::EitherOf12::A;
    |
      and 17 other candidates

error[E0425]: cannot find value `A` in this scope
   --> src/app.rs:300:22
    |
300 |                     <A class=move || format!("navlink{}", if is_active("/people") { " nb-active" } else { "" }) href="/people">"people"</A>
    |                      ^ not found in this scope
    |
help: consider importing one of these items
    |
1   + use crate::app::components::A;
    |
1   + use crate::app::either::EitherOf10::A;
    |
1   + use crate::app::either::EitherOf11::A;
    |
1   + use crate::app::either::EitherOf12::A;
    |
      and 17 other candidates

error[E0425]: cannot find value `A` in this scope
   --> src/app.rs:301:22
    |
301 |                     <A class=move || format!("navlink{}", if is_active("/blog") { " nb-active" } else { "" }) href="/blog">"Blog"</A>
    |                      ^ not found in this scope
    |
help: consider importing one of these items
    |
1   + use crate::app::components::A;
    |
1   + use crate::app::either::EitherOf10::A;
    |
1   + use crate::app::either::EitherOf11::A;
    |
1   + use crate::app::either::EitherOf12::A;
    |
      and 17 other candidates

error[E0412]: cannot find type `ServerFnError` in this scope
   --> src/app.rs:347:63
    |
347 | pub async fn get_user_details() -> Result<Option<ActiveUser>, ServerFnError> {
    |                                                               ^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these enums
    |
1   + use crate::app::server_fn::ServerFnError;
    |
1   + use leptos::prelude::ServerFnError;
    |

error[E0425]: cannot find function `create_signal` in this scope
   --> src/app.rs:373:31
    |
373 |     let (get_user,set_user) = create_signal(
    |                               ^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::reactive::signal::create_signal;
    |
1   + use leptos::prelude::create_signal;
    |

error[E0433]: failed to resolve: use of undeclared type `Signal`
   --> src/app.rs:416:19
    |
416 |     let user_rb = Signal::derive(move || {
    |                   ^^^^^^ use of undeclared type `Signal`
    |
help: consider importing one of these structs
    |
1   + use crate::app::reactive::wrappers::read::Signal;
    |
1   + use leptos::prelude::Signal;
    |

error[E0425]: cannot find function `create_effect` in this scope
   --> src/app.rs:439:5
    |
439 |     create_effect(move |_| {
    |     ^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::reactive::effect::create_effect;
    |
1   + use leptos::prelude::create_effect;
    |

error[E0425]: cannot find function `create_signal` in this scope
   --> src/app.rs:465:30
    |
465 |     let (posts, set_posts) = create_signal(vec![
    |                              ^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::reactive::signal::create_signal;
    |
1   + use leptos::prelude::create_signal;
    |

error[E0425]: cannot find function `create_effect` in this scope
   --> src/app.rs:486:5
    |
486 |     create_effect(move |_| {
    |     ^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::reactive::effect::create_effect;
    |
1   + use leptos::prelude::create_effect;
    |

error[E0425]: cannot find value `Router` in this scope
   --> src/app.rs:524:10
    |
524 |         <Router>
    |          ^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Router;
    |
1   + use leptos_router::components::Router;
    |

error[E0425]: cannot find value `Routes` in this scope
   --> src/app.rs:526:14
    |
526 |             <Routes>
    |              ^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Routes;
    |
1   + use leptos_router::components::Routes;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:527:18
    |
527 |                 <Route path="/" view=HomePage />
    |                  ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:528:18
    |
528 |                 <Route path="/home" view=HomePage>
    |                  ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:529:22
    |
529 |                     <Route path="" view=|| view! { 
    |                      ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:532:22
    |
532 |                     <Route path="editing" view = move || view! { <Portf_editing /> } />
    |                      ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:533:22
    |
533 |                     <Route path="programming" view = move || view! { <Portf_programming posts /> } />
    |                      ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:534:22
    |
534 |                     <Route path=":id" view=move || view! { 
    |                      ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:537:26
    |
537 |                         <Route path="" view=|| view! { 
    |                          ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:543:18
    |
543 |                 <Route path="/testing" view=move || view! { <ControlledWriting get_user/> } /> 
    |                  ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:545:18
    |
545 |                 <Route path="/people" view=move || view! { <PeopleList people /> } />
    |                  ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:547:18
    |
547 |                 <Route path="/blog" view=move || view! { <PostList posts=posts /> }>
    |                  ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/app.rs:548:22
    |
548 |                     <Route path="newpost" view=move || view! { <ControlledWriting get_user/> } /> 
    |                      ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::app::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0412]: cannot find type `Errors` in this scope
  --> src/error_template.rs:24:46
   |
24 |     #[prop(optional)] outside_errors: Option<Errors>,
   |                                              ^^^^^^ not found in this scope
   |
help: consider importing one of these structs
   |
1  + use crate::error_template::error::Errors;
   |
1  + use leptos::error::Errors;
   |

error[E0412]: cannot find type `RwSignal` in this scope
  --> src/error_template.rs:25:38
   |
25 |     #[prop(optional)] errors: Option<RwSignal<Errors>>,
   |                                      ^^^^^^^^ not found in this scope
   |
help: consider importing one of these structs
   |
1  + use crate::error_template::reactive::signal::RwSignal;
   |
1  + use leptos::prelude::RwSignal;
   |

error[E0412]: cannot find type `Errors` in this scope
  --> src/error_template.rs:25:47
   |
25 |     #[prop(optional)] errors: Option<RwSignal<Errors>>,
   |                                               ^^^^^^ not found in this scope
   |
help: consider importing one of these structs
   |
1  + use crate::error_template::error::Errors;
   |
1  + use leptos::error::Errors;
   |

error[E0425]: cannot find function `create_rw_signal` in this scope
  --> src/error_template.rs:28:20
   |
28 |         Some(e) => create_rw_signal(e),
   |                    ^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing one of these functions
   |
1  + use crate::error_template::reactive::signal::create_rw_signal;
   |
1  + use leptos::prelude::create_rw_signal;
   |

error[E0425]: cannot find value `For` in this scope
  --> src/error_template.rs:57:10
   |
57 |         <For
   |          ^^^ not found in this scope
   |
help: consider importing one of these items
   |
1  + use crate::error_template::attr::For;
   |
1  + use crate::error_template::control_flow::For;
   |
1  + use leptos::attr::For;
   |
1  + use leptos::control_flow::For;
   |

error[E0425]: cannot find value `Outlet` in this scope
   --> src/modules/blog_posts/blog_compo.rs:160:10
    |
160 |         <Outlet/>
    |          ^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::modules::blog_posts::blog_compo::nested_router::Outlet;
    |
1   + use leptos_router::nested_router::Outlet;
    |

error[E0425]: cannot find function `use_params_map` in this scope
   --> src/modules/blog_posts/blog_compo.rs:169:18
    |
169 |     let params = use_params_map();
    |                  ^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::modules::blog_posts::blog_compo::hooks::use_params_map;
    |
1   + use leptos_router::hooks::use_params_map;
    |

error[E0425]: cannot find value `Outlet` in this scope
   --> src/modules/blog_posts/blog_compo.rs:211:10
    |
211 |         <Outlet/>
    |          ^^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::modules::blog_posts::blog_compo::nested_router::Outlet;
    |
1   + use leptos_router::nested_router::Outlet;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/modules/blog_posts/blog_compo.rs:221:10
    |
221 |         <Route path="/blog" view=move || view! { <PostList posts={posts} /> }>   
    |          ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::modules::blog_posts::blog_compo::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/modules/blog_posts/blog_compo.rs:222:14
    |
222 |             <Route path="" view=|| view! {
    |              ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::modules::blog_posts::blog_compo::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/modules/blog_posts/blog_compo.rs:225:14
    |
225 |             <Route  path=":id" view=move ||  view! { <PostInfo posts={posts} _set_posts={set_posts}/> }>
    |              ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::modules::blog_posts::blog_compo::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find value `Route` in this scope
   --> src/modules/blog_posts/blog_compo.rs:226:18
    |
226 |                 <Route path="" view=move || view! {
    |                  ^^^^^ not found in this scope
    |
help: consider importing one of these functions
    |
1   + use crate::modules::blog_posts::blog_compo::components::Route;
    |
1   + use leptos_router::components::Route;
    |

error[E0425]: cannot find function `spawn_local` in this scope
  --> src/modules/blog_posts/blog_fn.rs:11:5
   |
11 |     spawn_local(async move {
   |     ^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
1  + use leptos::task::spawn_local;
   |

error[E0425]: cannot find function `spawn_local` in this scope
  --> src/modules/blog_posts/blog_fn.rs:57:9
   |
57 |         spawn_local(async move {
   |         ^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
1  + use leptos::task::spawn_local;
   |

error[E0425]: cannot find function `spawn_local` in this scope
   --> src/modules/textfields/skrijf.rs:243:9
    |
243 |         spawn_local(async move {
    |         ^^^^^^^^^^^ not found in this scope
    |
help: consider importing this function
    |
1   + use leptos::task::spawn_local;
    |

error[E0425]: cannot find function `mount_to_body` in crate `leptos`
  --> src/lib.rs:13:13
   |
13 |     leptos::mount_to_body(App);
   |             ^^^^^^^^^^^^^ not found in `leptos`
   |
help: consider importing this function
   |
1  + use leptos::mount::mount_to_body;
   |
help: if you import `mount_to_body`, refer to it directly
   |
13 -     leptos::mount_to_body(App);
13 +     mount_to_body(App);
   |

warning: unused import: `web_sys::wasm_bindgen::JsCast`
 --> src/modules/blog_posts/blog_compo.rs:7:6
  |
7 |  use web_sys::wasm_bindgen::JsCast;
  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `leptos_router::*`
  --> src/modules/blog_posts/blog_compo.rs:37:9
   |
37 |     use leptos_router::*;
   |         ^^^^^^^^^^^^^^^^

warning: unused import: `reqwest::Error`
 --> src/modules/blog_posts/blog_comms.rs:1:5
  |
1 | use reqwest::Error;
  |     ^^^^^^^^^^^^^^

warning: unused import: `serde_json::Value`
 --> src/modules/textfields/skrijf.rs:8:5
  |
8 | use serde_json::Value;
  |     ^^^^^^^^^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
 --> src/modules/textfields/syntectl.rs:3:13
  |
3 | use serde::{Serialize, Deserialize};
  |             ^^^^^^^^^  ^^^^^^^^^^^

warning: unused import: `reqwest::Client`
 --> src/modules/auth_fc/auth_ta.rs:5:5
  |
5 | use reqwest::Client;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `http::HeaderMap`
   --> src/modules/auth_fc/cookie.rs:109:9
    |
109 |     use http::HeaderMap;
    |         ^^^^^^^^^^^^^^^

warning: unused import: `ReadSignal`
 --> src/modules/people/people_comms.rs:5:5
  |
5 |     ReadSignal,
  |     ^^^^^^^^^^

warning: unused import: `crate::modules::blog_posts::blog_compo::Post`
 --> src/modules/portfolio/home.rs:6:5
  |
6 | use crate::modules::blog_posts::blog_compo::Post;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
  --> src/modules/blog_posts/blog_compo.rs:39:46
   |
39 |     let (selected_tags, set_selected_tags) = create_signal(Some(Vec::<String>::new()));
   |                                              ^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
  --> src/modules/blog_posts/blog_compo.rs:43:60
   |
43 |     let (previous_unique_tags, set_previous_unique_tags) = create_signal(Some(Vec::<String>::new()));
   |                                                            ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
  --> src/modules/blog_posts/blog_compo.rs:44:54
   |
44 |     let (disappearing_tags, set_disappearing_tags) = create_signal(Some(Vec::<String>::new()));
   |                                                      ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_memo`: This function is being removed to conform to Rust idioms. Please use `Memo::new()` instead.
   --> src/modules/blog_posts/blog_compo.rs:170:14
    |
170 |     let id = create_memo(move |_| params.with(|params| params.get("id").cloned().unwrap_or_default()));
    |              ^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
   --> src/modules/blog_posts/blog_compo.rs:242:46
    |
242 |     let (selected_tags, set_selected_tags) = create_signal(Some(Vec::<String>::new()));
    |                                              ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
   --> src/modules/blog_posts/blog_compo.rs:246:60
    |
246 |     let (previous_unique_tags, set_previous_unique_tags) = create_signal(Some(Vec::<String>::new()));
    |                                                            ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
   --> src/modules/blog_posts/blog_compo.rs:247:54
    |
247 |     let (disappearing_tags, set_disappearing_tags) = create_signal(Some(Vec::<String>::new()));
    |                                                      ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
 --> src/modules/blog_posts/blog_fn.rs:9:30
  |
9 |     let (posts, set_posts) = create_signal(vec![]);
  |                              ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
   --> src/modules/textfields/skrijf.rs:159:42
    |
159 |     let (post_status, set_post_status) = create_signal(None);
    |                                          ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
   --> src/modules/textfields/skrijf.rs:162:40
    |
162 |     let (area_title, set_area_title) = create_signal("enter title".to_string());
    |                                        ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
   --> src/modules/textfields/skrijf.rs:163:38
    |
163 |     let (area_tags, set_area_tags) = create_signal("".to_string());
    |                                      ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
   --> src/modules/textfields/skrijf.rs:164:48
    |
164 |     let (content_string, set_content_string) = create_signal("*enter content*".to_string());
    |                                                ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
   --> src/modules/textfields/skrijf.rs:165:28
    |
165 |     let (code, set_code) = create_signal("fn main() { println!(\"Hello, world!\"); }".to_string());
    |                            ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
   --> src/modules/textfields/skrijf.rs:166:40
    |
166 |     let (final_html, set_final_html) = create_signal("".to_string());
    |                                        ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
   --> src/modules/textfields/skrijf.rs:300:28
    |
300 |     let (name, set_name) = create_signal("Uncontrolled".to_string());
    |                            ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_node_ref`: This function is being removed to conform to Rust idioms. Please use `NodeRef::new()` instead.
   --> src/modules/textfields/skrijf.rs:301:44
    |
301 |     let input_element: NodeRef<Textarea> = create_node_ref();
    |                                            ^^^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
  --> src/modules/portfolio/home.rs:12:30
   |
12 |     let (count, set_count) = create_signal(0);
   |                              ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
  --> src/modules/portfolio/home.rs:66:38
   |
66 |     let (last_link, set_last_link) = create_signal(None::<String>);
   |                                      ^^^^^^^^^^^^^

warning: use of deprecated function `leptos::prelude::create_signal`: This function is being renamed to `signal()` to conform to Rust idioms.
  --> src/modules/portfolio/home.rs:71:50
   |
71 |             let (href_signal,_href_signal_set) = create_signal(href.clone());
   |                                                  ^^^^^^^^^^^^^

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:130:5
    |
130 |       view! {
    |  _____^
131 | |
132 | |         <div class="content">
...   |
165 | |         //<p>"Name is: " {password}</p>
166 | |     }
    | |_____^ method not found in `HtmlElement<Div, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::html::Div, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:130:5
    |
130 |       view! {
    |  _____^
131 | |
132 | |         <div class="content">
...   |
165 | |         //<p>"Name is: " {password}</p>
166 | |     }
    | |_____^ method not found in `HtmlElement<Form, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::html::Form, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `node_ref` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:137:21
    |
130 | /     view! {
131 | |
132 | |         <div class="content">
...   |
136 | |                     value=userName
137 | |                     node_ref=username_input
    | |                    -^^^^^^^^ method not found in `HtmlElement<Input, (Attr<Type, Static<"text">>, Attr<Value, _>), ()>`
    | |____________________|
    |
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/node_ref.rs:135:8
    |
135 |       fn node_ref(
    |          -------- the method is available for `leptos::html::HtmlElement<Input, (Attr<Type, leptos::tachys::view::static_types::Static<"text">>, Attr<leptos::attr::Value, _>), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `NodeRefAttribute` which provides `node_ref` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::NodeRefAttribute;
    |

error[E0599]: no method named `node_ref` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:148:21
    |
130 | /     view! {
131 | |
132 | |         <div class="content">
...   |
147 | |                     value=password
148 | |                     node_ref=password_input
    | |                    -^^^^^^^^ method not found in `HtmlElement<Input, (Attr<Type, Static<"password">>, Attr<Value, _>), ()>`
    | |____________________|
    |
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/node_ref.rs:135:8
    |
135 |       fn node_ref(
    |          -------- the method is available for `leptos::html::HtmlElement<Input, (Attr<Type, leptos::tachys::view::static_types::Static<"password">>, Attr<leptos::attr::Value, _>), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `NodeRefAttribute` which provides `node_ref` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::NodeRefAttribute;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:184:5
    |
184 |       view! {
    |  _____^
185 | |         <div class=hide_class style="position: absolute; top: 10px; right: 10px;">
186 | |             {
187 | |                 move ||{
...   |
266 | |         // Reactively display the user's name
267 | |     }
    | |_____^ method not found in `HtmlElement<Div, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::html::Div, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:191:25
    |
191 |                           view!{
    |  _________________________^
192 | |                             <span>{format!("Wrong password or username ; {}", user.name)}</span>
193 | |                             // make it only clickable every 20 seconds;
194 | |                             // note they can still cycle the token itself so maybe put a limiter on
...   |
221 | |                             }>"try_again"</button>
222 | |                         }.into_view()
    | |_________________________^ method not found in `HtmlElement<Span, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::html::Span, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:191:25
    |
191 |                           view!{
    |  _________________________^
192 | |                             <span>{format!("Wrong password or username ; {}", user.name)}</span>
193 | |                             // make it only clickable every 20 seconds;
194 | |                             // note they can still cycle the token itself so maybe put a limiter on
...   |
221 | |                             }>"try_again"</button>
222 | |                         }.into_view()
    | |_________________________^ method not found in `HtmlElement<Button, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<Button, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:226:25
    |
226 |                           view! { 
    |  _________________________^
227 | |                             <span>{format!("Welcome back, {}", user.name)}</span>
228 | |                             
229 | |                             // make this into a widget since you use it twice
...   |
251 | |                             }>"Log Out"</button>
252 | |                         }.into_view()
    | |_________________________^ method not found in `HtmlElement<Span, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::html::Span, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:226:25
    |
226 |                           view! { 
    |  _________________________^
227 | |                             <span>{format!("Welcome back, {}", user.name)}</span>
228 | |                             
229 | |                             // make this into a widget since you use it twice
...   |
251 | |                             }>"Log Out"</button>
252 | |                         }.into_view()
    | |_________________________^ method not found in `HtmlElement<Button, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<Button, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0277]: the trait bound `&fn(UncontrolledComponentProps) -> impl leptos::IntoView {UncontrolledComponent}: ComponentConstructor<_, _>` is not satisfied
   --> src/app.rs:255:25
    |
255 | /                         view! { 
256 | |                             <>
257 | |                     
258 | |                             <UncontrolledComponent set_user_l3=set_user_l2/> 
259 | |
260 | |                             </>
261 | |                         }.into_view()
    | |_________________________^ the trait `ComponentConstructor<_, _>` is not implemented for `&fn(UncontrolledComponentProps) -> impl leptos::IntoView {UncontrolledComponent}`
    |
note: required by a bound in `component_view`
   --> /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos-0.7.0/src/component.rs:59:37
    |
59  | pub fn component_view<P, T>(f: impl ComponentConstructor<P, T>, props: P) -> T {
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `component_view`
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `fn(UncontrolledComponentProps) -> impl leptos::IntoView {UncontrolledComponent}: leptos::prelude::Component<_>` is not satisfied
   --> src/app.rs:255:25
    |
255 | /                         view! { 
256 | |                             <>
257 | |                     
258 | |                             <UncontrolledComponent set_user_l3=set_user_l2/> 
259 | |
260 | |                             </>
261 | |                         }.into_view()
    | |_________________________^ the trait `leptos::prelude::Component<_>` is not implemented for fn item `fn(UncontrolledComponentProps) -> impl leptos::IntoView {UncontrolledComponent}`
    |
note: required by a bound in `component_props_builder`
   --> /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos-0.7.0/src/component.rs:54:15
    |
53  | pub fn component_props_builder<P: PropsOrNoPropsBuilder>(
    |        ----------------------- required by a bound in this function
54  |     _f: &impl Component<P>,
    |               ^^^^^^^^^^^^ required by this bound in `component_props_builder`
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:288:5
    |
288 |       view! {
    |  _____^
289 | |
290 | |         //all the navlinks
291 | |         <div>
...   |
339 | |         </div>
340 | |     }
    | |_____^ method not found in `HtmlElement<Div, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::html::Div, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:288:5
    |
288 |       view! {
    |  _____^
289 | |
290 | |         //all the navlinks
291 | |         <div>
...   |
339 | |         </div>
340 | |     }
    | |_____^ method not found in `HtmlElement<Nav, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<Nav, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:288:5
    |
288 |       view! {
    |  _____^
289 | |
290 | |         //all the navlinks
291 | |         <div>
...   |
339 | |         </div>
340 | |     }
    | |_____^ method not found in `HtmlElement<Button, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<Button, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:288:5
    |
288 |       view! {
    |  _____^
289 | |
290 | |         //all the navlinks
291 | |         <div>
...   |
339 | |         </div>
340 | |     }
    | |_____^ method not found in `HtmlElement<Svg, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<Svg, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:288:5
    |
288 |       view! {
    |  _____^
289 | |
290 | |         //all the navlinks
291 | |         <div>
...   |
339 | |         </div>
340 | |     }
    | |_____^ method not found in `HtmlElement<G, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::svg::G, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `class` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:324:35
    |
288 | /     view! {
289 | |
290 | |         //all the navlinks
291 | |         <div>
...   |
323 | |                         <g>
324 | |                             <path class="" data-original="#000000" fill="#595959" d="M256 0c-74.439 0-135 60.561-135 135s60.561 135 135 1...
    | |                                  -^^^^^ method not found in `HtmlElement<Path, (), ()>`
    | |__________________________________|
    |
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/attribute/global.rs:25:8
    |
25  |       fn class(self, value: C) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::svg::Path, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `ClassAttribute` which provides `class` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ClassAttribute;
    |

error[E0277]: the trait bound `&fn(UserElementProps) -> impl leptos::IntoView {UserElement}: ComponentConstructor<_, _>` is not satisfied
   --> src/app.rs:333:25
    |
333 |                         view! { <UserElement user_l2=user_l1 set_user_l2=set_user_l1 is_hiding_l=is_hiding /> }.into_view()
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ComponentConstructor<_, _>` is not implemented for `&fn(UserElementProps) -> impl leptos::IntoView {UserElement}`
    |
note: required by a bound in `component_view`
   --> /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos-0.7.0/src/component.rs:59:37
    |
59  | pub fn component_view<P, T>(f: impl ComponentConstructor<P, T>, props: P) -> T {
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `component_view`
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `fn(UserElementProps) -> impl leptos::IntoView {UserElement}: leptos::prelude::Component<_>` is not satisfied
   --> src/app.rs:333:25
    |
333 |                         view! { <UserElement user_l2=user_l1 set_user_l2=set_user_l1 is_hiding_l=is_hiding /> }.into_view()
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `leptos::prelude::Component<_>` is not implemented for fn item `fn(UserElementProps) -> impl leptos::IntoView {UserElement}`
    |
note: required by a bound in `component_props_builder`
   --> /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos-0.7.0/src/component.rs:54:15
    |
53  | pub fn component_props_builder<P: PropsOrNoPropsBuilder>(
    |        ----------------------- required by a bound in this function
54  |     _f: &impl Component<P>,
    |               ^^^^^^^^^^^^ required by this bound in `component_props_builder`
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: `if` and `else` have incompatible types
   --> src/app.rs:335:25
    |
332 |                       {move || if show_card.get() {
    |  ______________________________-
333 | |                         view! { <UserElement user_l2=user_l1 set_user_l2=set_user_l1 is_hiding_l=is_hiding /> }.into_view()
    | |                         --------------------------------------------------------------------------------------------------- expected because of this
334 | |                     } else {
335 | |                         view! { <></> }.into_view() // Empty view when not shown
    | |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `View<View<_>>`, found `View<()>`
336 | |                     }}
    | |_____________________- `if` and `else` have incompatible types
    |
    = note: expected struct `leptos::prelude::View<leptos::prelude::View<_>>`
               found struct `leptos::prelude::View<()>`

error[E0425]: cannot find function `create_resource` in this scope
   --> src/app.rs:382:32
    |
382 |     let async_UserCookieData = create_resource(
    |                                ^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find function `create_resource` in this scope
   --> src/app.rs:405:26
    |
405 |     let token_resource = create_resource(
    |                          ^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find function `create_resource` in this scope
   --> src/app.rs:475:28
    |
475 |     let async_data_posts = create_resource(
    |                            ^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find function `create_resource` in this scope
   --> src/app.rs:497:18
    |
497 |     let people = create_resource(
    |                  ^^^^^^^^^^^^^^^ not found in this scope

error[E0599]: no method named `lang` found for struct `EmptyPropsBuilder` in the current scope
   --> src/app.rs:522:15
    |
519 | /     view! {
520 | |         <Stylesheet id="leptos" href="/pkg/werk.css" />
521 | |         <Title text="Welcome to Leptos"/>
522 | |         <Html lang="eng" dir="ltr" attr:data-theme="dark"/>
    | |              -^^^^ method not found in `EmptyPropsBuilder`
    | |______________|
    |

error[E0599]: no method named `add_any_attr` found for opaque type `impl leptos::IntoView` in the current scope
   --> src/app.rs:519:5
    |
519 |       view! {
    |  _____^
520 | |         <Stylesheet id="leptos" href="/pkg/werk.css" />
521 | |         <Title text="Welcome to Leptos"/>
522 | |         <Html lang="eng" dir="ltr" attr:data-theme="dark"/>
...   |
555 | |         </Router>
556 | |     }
    | |_____^ method not found in `impl IntoView`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/view/add_attr.rs:16:8
    |
16  |       fn add_any_attr<NewAttr: Attribute>(
    |          ------------ the method is available for `impl leptos::IntoView` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `AddAnyAttr` which provides `add_any_attr` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::AddAnyAttr;
    |

error[E0277]: the trait bound `&fn(NavBarProps) -> impl leptos::IntoView {NavBar}: ComponentConstructor<_, _>` is not satisfied
   --> src/app.rs:519:5
    |
519 | /     view! {
520 | |         <Stylesheet id="leptos" href="/pkg/werk.css" />
521 | |         <Title text="Welcome to Leptos"/>
522 | |         <Html lang="eng" dir="ltr" attr:data-theme="dark"/>
...   |
555 | |         </Router>
556 | |     }
    | |_____^ the trait `ComponentConstructor<_, _>` is not implemented for `&fn(NavBarProps) -> impl leptos::IntoView {NavBar}`
    |
note: required by a bound in `component_view`
   --> /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos-0.7.0/src/component.rs:59:37
    |
59  | pub fn component_view<P, T>(f: impl ComponentConstructor<P, T>, props: P) -> T {
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `component_view`
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `fn(NavBarProps) -> impl leptos::IntoView {NavBar}: leptos::prelude::Component<_>` is not satisfied
   --> src/app.rs:519:5
    |
519 | /     view! {
520 | |         <Stylesheet id="leptos" href="/pkg/werk.css" />
521 | |         <Title text="Welcome to Leptos"/>
522 | |         <Html lang="eng" dir="ltr" attr:data-theme="dark"/>
...   |
555 | |         </Router>
556 | |     }
    | |_____^ the trait `leptos::prelude::Component<_>` is not implemented for fn item `fn(NavBarProps) -> impl leptos::IntoView {NavBar}`
    |
note: required by a bound in `component_props_builder`
   --> /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos-0.7.0/src/component.rs:54:15
    |
53  | pub fn component_props_builder<P: PropsOrNoPropsBuilder>(
    |        ----------------------- required by a bound in this function
54  |     _f: &impl Component<P>,
    |               ^^^^^^^^^^^^ required by this bound in `component_props_builder`
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:529:44
    |
529 |                       <Route path="" view=|| view! { 
    |  ____________________________________________^
530 | |                         <p>"Select a contact to view more info."</p> 
531 | |                     } />  // Nested route inside "/home"
    | |_____________________^ method not found in `HtmlElement<P, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::html::P, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:534:52
    |
534 |                       <Route path=":id" view=move || view! { 
    |  ____________________________________________________^
535 | |                         <div>"This is a test"</div> 
536 | |                     }>
    | |_____________________^ method not found in `HtmlElement<Div, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::html::Div, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/app.rs:537:48
    |
537 |                           <Route path="" view=|| view! { 
    |  ________________________________________________^
538 | |                             <p>"Select a contact to view more info."</p> 
539 | |                         } />  
    | |_________________________^ method not found in `HtmlElement<P, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::html::P, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/error_template.rs:55:5
    |
55  |       view! {
    |  _____^
56  | |         <h1>{if errors.len() > 1 {"Errors"} else {"Error"}}</h1>
57  | |         <For
58  | |             // a function that returns the items we're iterating over; a signal is fine
...   |
71  | |         />
72  | |     }
    | |_____^ method not found in `HtmlElement<H1, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<H1, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/error_template.rs:66:17
    |
66  |                   view! {
    |  _________________^
67  | |                     <h2>{error_code.to_string()}</h2>
68  | |                     <p>"Error: " {error_string}</p>
69  | |                 }
    | |_________________^ method not found in `HtmlElement<H2, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<H2, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0599]: no method named `child` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/error_template.rs:66:17
    |
66  |                   view! {
    |  _________________^
67  | |                     <h2>{error_code.to_string()}</h2>
68  | |                     <p>"Error: " {error_string}</p>
69  | |                 }
    | |_________________^ method not found in `HtmlElement<P, (), ()>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:140:8
    |
140 |       fn child(self, child: NewChild) -> Self::Output;
    |          ----- the method is available for `leptos::html::HtmlElement<leptos::html::P, (), ()>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)
help: trait `ElementChild` which provides `child` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::ElementChild;
    |

error[E0271]: expected `{closure@blog_compo.rs:114:72}` to be a closure that returns `View<HtmlElement<Button, (Class<&str>, On<click, {closure@blog_compo.rs:98:42}>), (String,)>>`, but it returns `View<HtmlElement<Button, (Class<Static<"tags disappearing">>, On<click, {closure@blog_compo.rs:120:42}>), (String,)>>`
   --> src/modules/blog_posts/blog_compo.rs:130:42
    |
98  |                                 on:click=move |_| {
    |                                          -------- the expected closure
...
120 |                                 on:click=move |_| {
    |                                          -------- the found closure
...
130 |                     unique_buttons.chain(disappearing_buttons).collect_view()
    |                                    ----- ^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `Static<"tags disappearing">`
    |                                    |
    |                                    required by a bound introduced by this call
    |
    = note: expected struct `leptos::prelude::View<leptos::html::HtmlElement<_, (Class<&str>, On<_, {closure@src/modules/blog_posts/blog_compo.rs:98:42: 98:50}>), _>>`
               found struct `View<HtmlElement<Button, (Class<Static<"tags disappearing">>, On<click, ...>), ...>>`
    = note: the full type name has been written to '/home/ghuber/dev/leptos/tfx_portal/pointseven/target/front/wasm32-unknown-unknown/debug/deps/home_portal.long-type-2054260126219712202.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: required for `std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>` to implement `std::iter::Iterator`
note: required by a bound in `std::iter::Iterator::chain`
   --> /home/ghuber/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:479:25
    |
476 |     fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter>
    |        ----- required by a bound in this associated function
...
479 |         U: IntoIterator<Item = Self::Item>,
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::chain`

error[E0599]: the method `collect_view` exists for struct `Chain<Map<Iter<'_, String>, {closure@blog_compo.rs:90:60}>, Map<Iter<'_, String>, ...>>`, but its trait bounds were not satisfied
   --> src/modules/blog_posts/blog_compo.rs:130:64
    |
130 |                     unique_buttons.chain(disappearing_buttons).collect_view()
    |                                                                ^^^^^^^^^^^^ method cannot be called due to unsatisfied trait bounds
    |
   ::: /home/ghuber/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:23:1
    |
23  | pub struct Chain<A, B> {
    | ---------------------- doesn't satisfy `<_ as IntoIterator>::Item = _`, `_: CollectView` or `_: IntoIterator`
    |
    = note: the full type name has been written to '/home/ghuber/dev/leptos/tfx_portal/pointseven/target/front/wasm32-unknown-unknown/debug/deps/home_portal.long-type-16373380508532547643.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: the following trait bounds were not satisfied:
            `<std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>> as IntoIterator>::Item = _`
            which is required by `std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>>: leptos::prelude::CollectView`
            `std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>>: IntoIterator`
            which is required by `std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>>: leptos::prelude::CollectView`
            `<&std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>> as IntoIterator>::Item = _`
            which is required by `&std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>>: leptos::prelude::CollectView`
            `&std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>>: IntoIterator`
            which is required by `&std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>>: leptos::prelude::CollectView`
            `<&mut std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>> as IntoIterator>::Item = _`
            which is required by `&mut std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>>: leptos::prelude::CollectView`
            `&mut std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>>: IntoIterator`
            which is required by `&mut std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:90:60: 90:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:114:72: 114:77}>>: leptos::prelude::CollectView`

error[E0277]: the trait bound `&std::string::String: IntoRender` is not satisfied
   --> src/modules/blog_posts/blog_compo.rs:153:44
    |
152 | /                         view! {
153 | |                             <A href={href}>{&post.title}</A>
    | |                                            ^-----------^
    | |                                            ||
    | |                                            |this tail expression is of type `&String`
    | |                                            the trait `Fn()` is not implemented for `std::string::String`, which is required by `&std::string::String: IntoRender`
154 | |                         }
    | |_________________________- required by a bound introduced by this call
    |
    = note: required for `&std::string::String` to implement `FnOnce()`
    = note: required for `&std::string::String` to implement `ReactiveFunction`
    = note: required for `&std::string::String` to implement `leptos::prelude::Render`
    = note: required for `&std::string::String` to implement `IntoRender`
help: you might have meant to create the closure instead of a block
    |
153 |                             <A href={href}>|_| {&post.title}</A>
    |                                            +++

error[E0599]: no method named `key` found for struct `HtmlElement` in the current scope
   --> src/modules/blog_posts/blog_compo.rs:184:40
    |
180 | /     view! {
181 | |         //<div class= "small_void_void"></div>
182 | |         <div class= "text_section">
183 | |             <div class= "skrijver_out decorated">
184 | |                 <h1 class="blog_title" key={id()}>
    | |                                       -^^^ method not found in `HtmlElement<H1, (Class<Static<"blog_title">>,), ({closure@blog_compo.rs:186:25},)>`
    | |_______________________________________|
    |
    |
    = note: the full type name has been written to '/home/ghuber/dev/leptos/tfx_portal/pointseven/target/front/wasm32-unknown-unknown/debug/deps/home_portal.long-type-2920789540327078916.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `key` found for struct `leptos::html::HtmlElement` in the current scope
   --> src/modules/blog_posts/blog_compo.rs:194:22
    |
180 | /     view! {
181 | |         //<div class= "small_void_void"></div>
182 | |         <div class= "text_section">
183 | |             <div class= "skrijver_out decorated">
...   |
193 | |                 </h1>
194 | |                 <div key={id()}>
    | |                     -^^^ method not found in `HtmlElement<Div, (), ({closure@blog_compo.rs:196:25},)>`
    | |_____________________|
    |

error[E0271]: expected `{closure@blog_compo.rs:316:72}` to be a closure that returns `View<HtmlElement<Button, (Class<&str>, On<click, {closure@blog_compo.rs:300:42}>), (String,)>>`, but it returns `View<HtmlElement<Button, (Class<Static<"tags disappearing">>, On<click, {closure@blog_compo.rs:322:42}>), (String,)>>`
   --> src/modules/blog_posts/blog_compo.rs:332:42
    |
300 |                                 on:click=move |_| {
    |                                          -------- the expected closure
...
322 |                                 on:click=move |_| {
    |                                          -------- the found closure
...
332 |                     unique_buttons.chain(disappearing_buttons).collect_view()
    |                                    ----- ^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `Static<"tags disappearing">`
    |                                    |
    |                                    required by a bound introduced by this call
    |
    = note: expected struct `leptos::prelude::View<leptos::html::HtmlElement<_, (Class<&str>, On<_, {closure@src/modules/blog_posts/blog_compo.rs:300:42: 300:50}>), _>>`
               found struct `View<HtmlElement<Button, (Class<Static<"tags disappearing">>, On<click, ...>), ...>>`
    = note: the full type name has been written to '/home/ghuber/dev/leptos/tfx_portal/pointseven/target/front/wasm32-unknown-unknown/debug/deps/home_portal.long-type-8919125265611204293.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: required for `std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>` to implement `std::iter::Iterator`
note: required by a bound in `std::iter::Iterator::chain`
   --> /home/ghuber/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:479:25
    |
476 |     fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter>
    |        ----- required by a bound in this associated function
...
479 |         U: IntoIterator<Item = Self::Item>,
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::chain`

error[E0599]: the method `collect_view` exists for struct `Chain<Map<Iter<'_, String>, {closure@blog_compo.rs:292:60}>, Map<Iter<'_, String>, ...>>`, but its trait bounds were not satisfied
   --> src/modules/blog_posts/blog_compo.rs:332:64
    |
332 |                     unique_buttons.chain(disappearing_buttons).collect_view()
    |                                                                ^^^^^^^^^^^^ method cannot be called due to unsatisfied trait bounds
    |
   ::: /home/ghuber/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/chain.rs:23:1
    |
23  | pub struct Chain<A, B> {
    | ---------------------- doesn't satisfy `<_ as IntoIterator>::Item = _`, `_: CollectView` or `_: IntoIterator`
    |
    = note: the full type name has been written to '/home/ghuber/dev/leptos/tfx_portal/pointseven/target/front/wasm32-unknown-unknown/debug/deps/home_portal.long-type-5391641558077300556.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: the following trait bounds were not satisfied:
            `<std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>> as IntoIterator>::Item = _`
            which is required by `std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>>: leptos::prelude::CollectView`
            `std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>>: IntoIterator`
            which is required by `std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>>: leptos::prelude::CollectView`
            `<&std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>> as IntoIterator>::Item = _`
            which is required by `&std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>>: leptos::prelude::CollectView`
            `&std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>>: IntoIterator`
            which is required by `&std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>>: leptos::prelude::CollectView`
            `<&mut std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>> as IntoIterator>::Item = _`
            which is required by `&mut std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>>: leptos::prelude::CollectView`
            `&mut std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>>: IntoIterator`
            which is required by `&mut std::iter::Chain<std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:292:60: 292:65}>, std::iter::Map<std::slice::Iter<'_, std::string::String>, {closure@src/modules/blog_posts/blog_compo.rs:316:72: 316:77}>>: leptos::prelude::CollectView`

error[E0277]: the trait bound `&std::string::String: IntoRender` is not satisfied
   --> src/modules/blog_posts/blog_compo.rs:356:68
    |
351 | /                         view! {
352 | |                             <div class= "skrijver_out decorated">
353 | |                                 <div class= "text_section">
354 | |                                     <div class="horcard">
355 | |                                         <div class="card-header">
356 | |                                             <h2 class="card-title">{&post.title}</h2>
    | |                                                                    ^-----------^
    | |                                                                    ||
    | |                                                                    |this tail expression is of type `&String`
    | |                                                                    the trait `Fn()` is not implemented for `std::string::String`, which is required by `&std::string::String: IntoRender`
...   |
374 | |                             </div>
375 | |                         }
    | |_________________________- required by a bound introduced by this call
    |
    = note: required for `&std::string::String` to implement `FnOnce()`
    = note: required for `&std::string::String` to implement `ReactiveFunction`
    = note: required for `&std::string::String` to implement `leptos::prelude::Render`
    = note: required for `&std::string::String` to implement `IntoRender`
help: you might have meant to create the closure instead of a block
    |
356 |                                             <h2 class="card-title">|_| {&post.title}</h2>
    |                                                                    +++

error[E0277]: the trait bound `&std::string::String: leptos::prelude::Render` is not satisfied
   --> src/modules/blog_posts/blog_compo.rs:351:25
    |
351 | /                         view! {
352 | |                             <div class= "skrijver_out decorated">
353 | |                                 <div class= "text_section">
354 | |                                     <div class="horcard">
...   |
374 | |                             </div>
375 | |                         }
    | |_________________________^ the trait `Fn()` is not implemented for `std::string::String`, which is required by `leptos::html::HtmlElement<H2, (), ()>: leptos::prelude::ElementChild<_>`
    |
    = help: the trait `leptos::prelude::ElementChild<NewChild>` is implemented for `leptos::html::HtmlElement<E, At, Ch>`
    = note: required for `&std::string::String` to implement `FnOnce()`
    = note: required for `&std::string::String` to implement `ReactiveFunction`
    = note: required for `&std::string::String` to implement `leptos::prelude::Render`
    = note: required for `leptos::html::HtmlElement<H2, (), ()>` to implement `leptos::prelude::ElementChild<&std::string::String>`
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: the method `class` exists for struct `HtmlElement<H2, (), (&String,)>`, but its trait bounds were not satisfied
   --> src/modules/blog_posts/blog_compo.rs:356:49
    |
351 | /                         view! {
352 | |                             <div class= "skrijver_out decorated">
353 | |                                 <div class= "text_section">
354 | |                                     <div class="horcard">
355 | |                                         <div class="card-header">
356 | |                                             <h2 class="card-title">{&post.title}</h2>
    | |                                                -^^^^^ method cannot be called on `HtmlElement<H2, (), (&String,)>` due to unsatisfied trait bounds
    | |________________________________________________|
    |
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tachys-0.1.0/src/html/element/mod.rs:34:1
    |
34  |   pub struct HtmlElement<E, At, Ch> {
    |   --------------------------------- doesn't satisfy `_: AsRef<Element>`, `_: ClassAttribute<_>` or `_: ElementExt`
    |
    = note: the following trait bounds were not satisfied:
            `(&std::string::String,): RenderHtml`
            which is required by `leptos::html::HtmlElement<H2, (), (&std::string::String,)>: leptos::prelude::ClassAttribute<_>`
            `leptos::html::HtmlElement<H2, (), (&std::string::String,)>: AsRef<web_sys::Element>`
            which is required by `leptos::html::HtmlElement<H2, (), (&std::string::String,)>: leptos::prelude::ElementExt`

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/modules/textfields/skrijf.rs:171:66
    |
171 |     let Allstat_resource = create_resource(content_string, move |content_string| async move {
    |                                                                  ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `Sized` is not implemented for `str`
    = note: all function arguments must have a statically known size
    = help: unsized fn params are gated as an unstable feature

error[E0425]: cannot find function `create_resource` in this scope
   --> src/modules/textfields/skrijf.rs:171:28
    |
171 |       let Allstat_resource = create_resource(content_string, move |content_string| async move {
    |                              ^^^^^^^^^^^^^^^ help: a function with a similar name exists: `create_slice`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reactive_graph-0.1.0/src/computed.rs:89:1
    |
89  | / pub fn create_slice<T, O, S>(
90  | |     signal: RwSignal<T>,
91  | |     getter: impl Fn(&T) -> O + Copy + Send + Sync + 'static,
92  | |     setter: impl Fn(&mut T, S) + Copy + Send + Sync + 'static,
...   |
95  | |     T: Send + Sync + 'static,
96  | |     O: PartialEq + Send + Sync + 'static,
    | |_________________________________________- similarly named function `create_slice` defined here

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/modules/textfields/skrijf.rs:178:64
    |
178 |     let Final_resource = create_resource(content_string, move |content_string| async move {
    |                                                                ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `Sized` is not implemented for `str`
    = note: all function arguments must have a statically known size
    = help: unsized fn params are gated as an unstable feature

error[E0425]: cannot find function `create_resource` in this scope
   --> src/modules/textfields/skrijf.rs:178:26
    |
178 |       let Final_resource = create_resource(content_string, move |content_string| async move {
    |                            ^^^^^^^^^^^^^^^ help: a function with a similar name exists: `create_slice`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reactive_graph-0.1.0/src/computed.rs:89:1
    |
89  | / pub fn create_slice<T, O, S>(
90  | |     signal: RwSignal<T>,
91  | |     getter: impl Fn(&T) -> O + Copy + Send + Sync + 'static,
92  | |     setter: impl Fn(&mut T, S) + Copy + Send + Sync + 'static,
...   |
95  | |     T: Send + Sync + 'static,
96  | |     O: PartialEq + Send + Sync + 'static,
    | |_________________________________________- similarly named function `create_slice` defined here

error[E0308]: `match` arms have incompatible types
   --> src/modules/textfields/skrijf.rs:283:21
    |
274 | /             match post_status.get().as_deref() {
275 | |                 Some("success") => {
276 | |                     // Start a timeout to clear the "success" message
277 | |                     Timeout::new(2_000, move || set_post_status(None)).forget();
278 | |                     view! { <div class="overlay success">"Post created successfully!"</div> }
    | |                     ------------------------------------------------------------------------- this is found to be of type `leptos::prelude::View<leptos::html::HtmlElement<leptos::html::Div, (Class<leptos::tachys::view::static_types::Static<"overlay success">>,), (leptos::tachys::view::static_types::Static<"Post created successfully!">,)>>`
...   |
283 | |                     view! { <div class="overlay error">"Failed to create post. Try again."</div> }
    | |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `"overlay success"`, found `"overlay error"`
284 | |                 },
285 | |                 _ => view! { <div></div> }, // Render empty div for consistency
286 | |             }
    | |_____________- `match` arms have incompatible types
    |
    = note: expected struct `View<HtmlElement<Div, (Class<Static<"overlay success">>,), (Static<"Post created successfully!">,)>>`
               found struct `View<HtmlElement<Div, (Class<Static<"overlay error">>,), (Static<"Failed to create post. Try again.">,)>>`
    = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0618]: expected function, found `leptos::prelude::NodeRef<Textarea>`
   --> src/modules/textfields/skrijf.rs:309:21
    |
301 |     let input_element: NodeRef<Textarea> = create_node_ref();
    |         ------------- `input_element` has type `leptos::prelude::NodeRef<Textarea>`
...
309 |         let value = input_element()
    |                     ^^^^^^^^^^^^^--
    |                     |
    |                     call expression requires function

error[E0599]: no method named `value` found for struct `leptos::html::HtmlElement<Textarea, (), ()>` in the current scope
   --> src/modules/textfields/skrijf.rs:319:17
    |
315 | /     view! {
316 | |         <form on:submit=on_submit>
317 | |             <textarea
318 | |                 // Set the initial value of the textarea
319 | |                 value=name
    | |                -^^^^^ method not found in `HtmlElement<Textarea, (), ()>`
    | |________________|
    |
    |
    = note: the method was found for
            - `leptos::html::HtmlElement<Button, At, Ch>`
            - `leptos::html::HtmlElement<Input, At, ()>`
            - `leptos::html::HtmlElement<Li, At, Ch>`
            - `leptos::html::HtmlElement<Meter, At, Ch>`
            and 3 more types

error[E0599]: no method named `iter` found for unit type `()` in the current scope
  --> src/modules/people/people_main.rs:33:37
   |
33 |                         people_list.iter().map(|person| {
   |                                     ^^^^ method not found in `()`

error[E0308]: `if` and `else` have incompatible types
  --> src/modules/people/people_main.rs:72:9
   |
62 | /       if let Some(pic_data) = profile_pic {
63 | |           // Encode profile picture data as Base64
64 | |           let encoded = general_purpose::STANDARD.encode(pic_data);
...  |
67 | | /         view! {
68 | | |             <img src={format!("data:image/jpeg;base64,{}", encoded)} alt="Profile Picture" />
69 | | |         }.into_view()
   | | |_____________________- expected because of this
...  |
72 | | /         view! {
73 | | |             <img src="https://cdn.pixabay.com/photo/2017/07/18/23/23/user-2517433_640.png" />
74 | | |         }.into_view()
   | | |_____________________^ expected a tuple with 2 elements, found one with 1 element
75 | |       }
   | |_______- `if` and `else` have incompatible types
   |
   = note: expected struct `View<View<HtmlElement<Img, (Attr<Src, String>, Attr<Alt, Static<"Profile Picture">>), ()>>>`
              found struct `View<View<HtmlElement<Img, (Attr<Src, ...>,), ...>>>`
help: you could change the return type to be a boxed trait object
   |
61 | fn render_profile_pic(profile_pic: &Option<Vec<u8>>) -> Box<dyn IntoView> {
   |                                                         ~~~~~~~         +
help: if you change the return type to expect trait objects, box the returned expressions
   |
67 ~         Box::new(view! {
68 |             <img src={format!("data:image/jpeg;base64,{}", encoded)} alt="Profile Picture" />
69 ~         }.into_view())
70 |     } else {
71 |         // Fallback text if no image data is available
72 ~         Box::new(view! {
73 |             <img src="https://cdn.pixabay.com/photo/2017/07/18/23/23/user-2517433_640.png" />
74 ~         }.into_view())
   |

error[E0308]: `if` and `else` have incompatible types
   --> src/modules/people/people_main.rs:117:49
    |
107 | /  ...                   if let (Some(key), Some(value)) = (pair_vec.get(0), pair_vec.get(1)) {
108 | |  ...                       let key_text = key.as_str().unwrap_or_default().to_string();
109 | |  ...                       let value_text = value.as_str().unwrap_or_default().to_string();
110 | |/ ...                       view! {
111 | || ...                           //<div class="card_plain">
112 | || ...                               <div class= "info_type">{key_text}:</div>
113 | || ...                               <div class= "value">{ value_text }</div>
114 | || ...                           //</div>
115 | || ...                       }.into_view()
    | ||_______________________________________- expected because of this
116 | |  ...                   } else {
117 | |  ...                       ().into_view()
    | |                            ^^^^^^^^^^^^^^ expected `View<View<(..., ...)>>`, found `View<()>`
118 | |  ...                   }
    | |________________________- `if` and `else` have incompatible types
    |
    = note: expected struct `View<View<(HtmlElement<Div, (Class<Static<"info_type">>,), ((String, Static<":">),)>, ...)>>`
               found struct `leptos::prelude::View<()>`
    = note: the full type name has been written to '/home/ghuber/dev/leptos/tfx_portal/pointseven/target/front/wasm32-unknown-unknown/debug/deps/home_portal.long-type-7735466606322464662.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0308]: `match` arms have incompatible types
   --> src/modules/people/people_main.rs:152:74
    |
144 |                                                                match platform_name.as_str() {
    |                                                                ---------------------------- `match` arms have incompatible types
145 |                                                               "instagram" => view! {
    |  ____________________________________________________________________________-
146 | |                                                                 <svg height="30px" width="30px" viewBox="0 0 256 256" class="instagram">
147 | |                                                                     <g transform="scale(8,8)">
148 | |                                                                         <path d="M11.46875,5c-3.55078,0 -6.46875,2.91406 -6.46875,6.46875...
149 | |                                                                     </g>
150 | |                                                                 </svg>
151 | |                                                             }.into_view(),
    | |_________________________________________________________________________- this is found to be of type `leptos::prelude::View<leptos::prelude::View<leptos::html::HtmlElement<Svg, (Class<leptos::tachys::view::static_types::Static<"instagram">>, CustomAttr<&str, leptos::tachys::view::static_types::Static<"30px">>, CustomAttr<&str, leptos::tachys::view::static_types::Static<"30px">>, CustomAttr<&str, leptos::tachys::view::static_types::Static<"0 0 256 256">>), (leptos::html::HtmlElement<leptos::svg::G, (CustomAttr<&str, leptos::tachys::view::static_types::Static<"scale(8,8)">>,), (leptos::html::HtmlElement<leptos::svg::Path, (CustomAttr<&str, leptos::tachys::view::static_types::Static<"M11.46875,5c-3.55078,0 -6.46875,2.91406 -6.46875,6.46875v9.0625c0,3.55078 2.91406,6.46875 6.46875,6.46875h9.0625c3.55078,0 6.46875,-2.91406 6.46875,-6.46875v-9.0625c0,-3.55078 -2.91406,-6.46875 -6.46875,-6.46875zM11.46875,7h9.0625c2.47266,0 4.46875,1.99609 4.46875,4.46875v9.0625c0,2.47266 -1.99609,4.46875 -4.46875,4.46875h-9.0625c-2.47266,0 -4.46875,-1.99609 -4.46875,-4.46875v-9.0625c0,-2.47266 1.99609,-4.46875 4.46875,-4.46875zM21.90625,9.1875c-0.50391,0 -0.90625,0.40234 -0.90625,0.90625c0,0.50391 0.40234,0.90625 0.90625,0.90625c0.50391,0 0.90625,-0.40234 0.90625,-0.90625c0,-0.50391 -0.40234,-0.90625 -0.90625,-0.90625zM16,10c-3.30078,0 -6,2.69922 -6,6c0,3.30078 2.69922,6 6,6c3.30078,0 6,-2.69922 6,-6c0,-3.30078 -2.69922,-6 -6,-6zM16,12c2.22266,0 4,1.77734 4,4c0,2.22266 -1.77734,4 -4,4c-2.22266,0 -4,-1.77734 -4,-4c0,-2.22266 1.77734,-4 4,-4z">>,), ()>,)>,)>>>`
152 |                                                               "twitter" => view! {
    |  __________________________________________________________________________^
153 | |                                                                 <svg height="30px" width="30px" viewBox="0 0 48 48" class="twitter">
154 | |                                                                     <path d="M42,12.429c-1.323,0.586-2.746,0.977-4.247,1.162c1.526-0.906,...
155 | |                                                                 </svg>
156 | |                                                             }.into_view(),
    | |_________________________________________________________________________^ expected `"instagram"`, found `"twitter"`
    |
    = note: expected struct `View<View<HtmlElement<Svg, (Class<Static<"instagram">>, CustomAttr<&str, Static<"30px">>, ..., ...), ...>>>`
               found struct `View<View<HtmlElement<Svg, (Class<Static<"twitter">>, CustomAttr<&str, Static<"30px">>, ..., ...), ...>>>`

error[E0599]: no method named `get` found for struct `Signal` in the current scope
   --> src/modules/people/people_comms.rs:35:33
    |
35  |     let access_token = get_user.get().unwrap().token;
    |                                 ^^^ method not found in `Signal<Option<ActiveUser>>`
    |
   ::: /home/ghuber/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reactive_graph-0.1.0/src/traits.rs:386:8
    |
386 |     fn get(&self) -> Self::Value {
    |        --- the method is available for `Signal<std::option::Option<ActiveUser>>` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `Get` which provides `get` is implemented but not in scope; perhaps you want to import it
    |
1   + use leptos::prelude::Get;
    |

error[E0277]: the trait bound `&UncontrolledComponentProps: Into<UncontrolledComponentProps>` is not satisfied
  --> src/app.rs:60:1
   |
60 | #[component]
   | ^^^^^^^^^^^^ the trait `From<&UncontrolledComponentProps>` is not implemented for `UncontrolledComponentProps`, which is required by `&UncontrolledComponentProps: Into<_>`
   |
   = note: required for `&UncontrolledComponentProps` to implement `Into<UncontrolledComponentProps>`
   = note: this error originates in the derive macro `::leptos::typed_builder_macro::TypedBuilder` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `&UserElementProps: Into<UserElementProps>` is not satisfied
   --> src/app.rs:170:1
    |
170 | #[component]
    | ^^^^^^^^^^^^ the trait `From<&UserElementProps>` is not implemented for `UserElementProps`, which is required by `&UserElementProps: Into<_>`
    |
    = note: required for `&UserElementProps` to implement `Into<UserElementProps>`
    = note: this error originates in the derive macro `::leptos::typed_builder_macro::TypedBuilder` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `&NavBarProps: Into<NavBarProps>` is not satisfied
   --> src/app.rs:274:1
    |
274 | #[component]
    | ^^^^^^^^^^^^ the trait `From<&NavBarProps>` is not implemented for `NavBarProps`, which is required by `&NavBarProps: Into<_>`
    |
    = note: required for `&NavBarProps` to implement `Into<NavBarProps>`
    = note: this error originates in the derive macro `::leptos::typed_builder_macro::TypedBuilder` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `__outside_errors: Optional<_>` is not satisfied
  --> src/error_template.rs:24:23
   |
22 | #[component]
   | ------------ required by a bound introduced by this call
23 | pub fn ErrorTemplate(
24 |     #[prop(optional)] outside_errors: Option<Errors>,
   |                       ^^^^^^^^^^^^^^ the trait `Optional<_>` is not implemented for `__outside_errors`
   |
   = help: the following other types implement trait `Optional<T>`:
             ()
             (T,)

error[E0277]: the trait bound `__errors: Optional<_>` is not satisfied
  --> src/error_template.rs:25:23
   |
22 | #[component]
   | ------------ required by a bound introduced by this call
...
25 |     #[prop(optional)] errors: Option<RwSignal<Errors>>,
   |                       ^^^^^^ the trait `Optional<_>` is not implemented for `__errors`
   |
   = help: the following other types implement trait `Optional<T>`:
             ()
             (T,)

warning: unused import: `leptos_router`
 --> src/app.rs:3:5
  |
3 | use leptos_router::*;
  |     ^^^^^^^^^^^^^

warning: unused import: `leptos_router`
 --> src/modules/blog_posts/blog_compo.rs:2:5
  |
2 | use leptos_router::*;
  |     ^^^^^^^^^^^^^

Some errors have detailed explanations: E0271, E0277, E0308, E0412, E0425, E0432, E0433, E0599, E0618.
For more information about an error, try `rustc --explain E0271`.
warning: `home_portal` (lib) generated 30 warnings
error: could not compile `home_portal` (lib) due to 138 previous errors; 30 warnings emitted
[38;5;77m      Notify[0m watching paths [38;5;241mstyle, src, public[0m
[38;5;77m      Leptos[0m ctrl-c received
