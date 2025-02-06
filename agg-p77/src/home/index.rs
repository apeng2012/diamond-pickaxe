use crate::Route;

use dioxus::prelude::*;

pub fn home() -> Element {
    rsx! {
        style { {include_str!("./../css/bootstrap.css")} }
        h3 { "I am the content of Home" }
        div {
            ul { class: "nav nav-tabs",
                li {
                    Link { class: "list-group-item", to: Route::News {}, "News" }
                }
                li {
                    Link {
                        class: "list-group-item",
                        to: Route::Detail {
                            id: "01".to_string(),
                            title: "message1".to_string(),
                        },
                        "Message"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
