#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./index.css") }
        div { class: "todo-footer",
            label { input { "type": "checkbox" } }
            span {
                span { "Completed 0" }
                "/ All 2"
            }
            button { class: "btn btn-danger", "Clear Completed Tasks" }
        }
    })
}
