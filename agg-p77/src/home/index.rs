use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn home(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./../css/bootstrap.css") }
        h3 { "I am the content of Home" }
        div {
            ul { class: "nav nav-tabs",
                li {
                    Link { class: "list-group-item", to: Route::News {}, "News" }
                }
                li {
                    Link { class: "list-group-item", to: Route::Message {}, "Message" }
                }
            }
        }
        Outlet::<Route> {}
    })
}
