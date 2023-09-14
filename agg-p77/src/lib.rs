#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod about;
mod home;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/home")]
        Home {},
        #[route("/about")]
        About {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

pub fn app(cx: Scope) -> Element {
    render!( Router::<Route> {} )
}

#[inline_props]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    Link { to: Route::About {}, "About" }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    render! { home::index::home {} }
}

#[inline_props]
fn About(cx: Scope) -> Element {
    render! { about::index::about {} }
}

#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
