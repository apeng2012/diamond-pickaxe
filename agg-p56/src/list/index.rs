use dioxus::prelude::*;

use crate::item::index::Item;
use crate::TodoProps;

#[component]
pub fn List(props: TodoProps) -> Element {
    let todos_vec = props.todos.iter().map(|f| *f.0).collect::<Vec<_>>();

    rsx! {
        style { {include_str!("./index.css")} }
        ul { class: "todo-main",
            {todos_vec.iter().map(|id| rsx! {
                Item { id: *id, set_todos: props.todos.clone() }
            })}
        }
    }
}
