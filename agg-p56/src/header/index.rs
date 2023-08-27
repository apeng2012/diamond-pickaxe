#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./index.css") }
        div { class: "todo-header",
            input {
                "type": "text",
                placeholder: "Please enter your task name and press Enter to confirm."
            }
        }
    })
}
