#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Item(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./index.css") }
        li {
            label {
                input { "type": "checkbox" }
                span { "xxxxx" }
            }
            button { class: "btn btn-danger", style: "display: none", "shanchu" }
        }
    })
}
