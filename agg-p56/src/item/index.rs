#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::Todo;

#[inline_props]
pub fn Item(cx: Scope, _id: i32, todo: Todo) -> Element {
    cx.render(rsx! {
        style { include_str!("./index.css") }
        li { key: "{_id}",
            label {
                input { "type": "checkbox", checked: "{todo.done}" }
                span { "{todo.name}" }
            }
            button { class: "btn btn-danger", style: "display: none", "shanchu" }
        }
    })
}
