#![allow(non_snake_case)]
use dioxus::prelude::*;
use im_rc::HashMap;

use crate::Todo;

#[inline_props]
pub fn Item<'a>(cx: Scope, id: i32, set_todos: &'a UseRef<HashMap<i32, Todo>>) -> Element {
    let mouse = use_state(cx, || false);
    let todo = &set_todos.read()[id];

    cx.render(rsx! {
        style { include_str!("./index.css") }
        li {
            background_color: if **mouse { "#ddd" } else { "white" },
            onmouseenter: |_| {
                mouse.set(true);
            },
            onmouseleave: |_| {
                mouse.set(false);
            },
            label {
                input {
                    "type": "checkbox",
                    checked: "{todo.done}",
                    oninput: move |event| {
                        set_todos.write()[id].done = event.value.parse().unwrap();
                    }
                }
                span { "{todo.name}" }
            }
            button { class: "btn btn-danger", style: if **mouse { "display: block" } else { "display: none" }, "shanchu" }
        }
    })
}
