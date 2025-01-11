use dioxus::prelude::*;
use im_rc::HashMap;

use crate::Todo;

#[component]
pub fn Item(id: usize, set_todos: HashMap<usize, Todo>) -> Element {
    let mut mouse = use_signal(|| false);
    let todo = &set_todos[&id];
    let mut set_todos_clone = set_todos.clone();

    rsx! {
        style { {include_str!("./index.css")} }
        li {
            background_color: if mouse() { "#ddd" } else { "white" },
            onmouseenter: move |_| {
                mouse.set(true);
            },
            onmouseleave: move |_| {
                mouse.set(false);
            },
            label {
                input {
                    "type": "checkbox",
                    checked: "{todo.done}",
                    oninput: move |event| {
                        set_todos[&id].done = event.value().parse().unwrap();
                    },
                }
                span { "{todo.name}" }
            }
            button {
                class: "btn btn-danger",
                style: if mouse() { "display: block" } else { "display: none" },
                onclick: move |_| {
                    set_todos_clone.remove(&id);
                },
                "Delete"
            }
        }
    }
}
