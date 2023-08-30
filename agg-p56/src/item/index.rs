#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::Todo;

#[inline_props]
pub fn Item(cx: Scope, _id: i32, todo: Todo) -> Element {
    let mouse = use_state(cx, || false);

    cx.render(rsx! {
        style { include_str!("./index.css") }
        li {
            key: "{_id}",
            background_color: if **mouse { "#ddd" } else { "white" },
            onmouseenter: |_| {
                mouse.set(true);
            },
            onmouseleave: |_| {
                mouse.set(false);
            },
            label {
                input { "type": "checkbox", checked: "{todo.done}" }
                span { "{todo.name}" }
            }
            button { class: "btn btn-danger", style: if **mouse { "display: block" } else { "display: none" }, "shanchu" }
        }
    })
}
