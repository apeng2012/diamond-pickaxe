use dioxus::prelude::*;
use im_rc::hashmap;

mod footer;
mod header;
mod item;
mod list;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let todos = use_signal(|| {
        hashmap! {
            1usize => Todo {
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

    rsx! {
        style { {include_str!("./main.css")} }
        div { class: "todo-container",
            div { class: "todo-wrap",
                header::index::Header { todos }
                list::index::List { todos }
                footer::index::Footer { todos }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Todo {
    name: String,
    done: bool,
}
