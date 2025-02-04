#![allow(non_snake_case)]

use dioxus::prelude::*;

mod about;
// mod home;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    About {},
    // #[nest("/home")]
    //     #[layout(Home)]
    //     #[nest("/message")]
    //         #[layout(Message)]
    //         #[route("/:id/:title")]
    //         Detail {
    //             id: String,
    //             title: String,
    //         },
    //         #[end_layout]
    //     #[end_nest]
    //     #[route("/news")]
    //     News {},
    //     #[end_layout]
    // #[end_nest]
    #[redirect("/about", || Route::About {})]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
pub fn app() -> Element {
    rsx!(
        Router::<Route> {}
    )
}

#[component]
fn NavBar() -> Element {
    let nav = navigator();

    rsx! {
        style { {include_str!("./css/bootstrap.css")} }
        div {
            div { class: "row",
                div { class: "col-xs-offset-2 col-xs-8",
                    div { class: "page-header",
                        h2 { "Dioxus Router Demo" }
                        button {
                            onclick: move |_| {
                                nav.go_back();
                            },
                            "Go back"
                        }
                        button {
                            onclick: move |_| {
                                nav.go_forward();
                            },
                            "Go forward"
                        }
                    }
                }
            }
            div { class: "row",
                div { class: "col-xs-2 col-xs-offset-2",
                    div { class: "list-group",
                        //                        Link { class: "list-group-item", to: Route::News {}, "Home" }
                        Link { class: "list-group-item", to: Route::About {}, "About" }
                    }
                }
                div { class: "col-xs-6",
                    div { class: "panel",
                        div { class: "panel-body",
                            div { Outlet::<Route> {} }
                        }
                    }
                }
            }
        }
    }
}

// #[inline_props]
// fn Home(cx: Scope) -> Element {
//     render! { home::index::home {} }
// }

// #[inline_props]
// fn Message(cx: Scope) -> Element {
//     render! { home::message::message {} }
// }

// #[inline_props]
// fn Detail(cx: Scope, id: String, title: String) -> Element {
//     render! { home::detail::detail { id: id.clone(), title: title.clone() } }
// }

// #[inline_props]
// fn News(cx: Scope) -> Element {
//     render! { home::news::news {} }
// }

#[component]
fn About() -> Element {
    rsx! {
        about::index::about {}
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
