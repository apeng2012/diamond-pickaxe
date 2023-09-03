#![allow(non_snake_case)]
use dioxus::prelude::*;
use im_rc::hashmap;

mod footer;
mod header;
mod item;
mod list;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let todos = use_ref(cx, || {
        hashmap! {
            1 => Todo {
                name: "Eat".to_string(),
                done: true,
            },
            2 => Todo {
                name: "Sleep".to_string(),
                done: true,
            },
            3 => Todo {
                name: "Code".to_string(),
                done: false,
            },
            4 => Todo {
                name: "Shop".to_string(),
                done: true,
            },
        }
    });

    cx.render(rsx! {
        style { include_str!("./main.css") }
        div { class: "todo-container",
            div { class: "todo-wrap",
                header::index::Header { todos: todos }
                list::index::List { todos: todos }
                footer::index::Footer { todos: todos }
            }
        }
    })
}

#[derive(Clone, PartialEq)]
pub struct Todo {
    name: String,
    done: bool,
}
