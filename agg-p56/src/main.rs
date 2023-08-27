#![allow(non_snake_case)]
use dioxus::prelude::*;

mod footer;
mod header;
mod item;
mod list;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./main.css") }
        div { class: "todo-container",
            div { class: "todo-wrap",
                header::index::Header {}
                list::index::List {}
                footer::index::Footer {}
            }
        }
    })
}
