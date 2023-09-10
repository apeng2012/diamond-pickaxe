#![allow(non_snake_case)]
use dioxus::prelude::*;

mod list;
mod search;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./css/bootstrap.css") }
        div { class: "container",
            search::index::search {}
            list::index::head_list {}
        }
    })
}
