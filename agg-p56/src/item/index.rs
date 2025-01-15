use dioxus::prelude::*;
use im_rc::HashMap;

use crate::Todo;

#[component]
pub fn Item(id: usize, mut set_todos: Signal<HashMap<usize, Todo>>) -> Element {
    let mut mouse = use_signal(|| false);
    let done = use_memo(move || set_todos.read().get(&id).unwrap().done);
    let name = use_memo(move || set_todos.read().get(&id).unwrap().name.clone());

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
                    checked: "{done}",
                    oninput: move |event| {
                        set_todos.write().get_mut(&id).unwrap().done = event.value().parse().unwrap();
                    },
                }
                span { "{name}" }
            }
            button {
                class: "btn btn-danger",
                style: if mouse() { "display: block" } else { "display: none" },
                onclick: move |_| {
                    set_todos.write().remove(&id);
                },
                "Delete"
            }
        }
    }
}
