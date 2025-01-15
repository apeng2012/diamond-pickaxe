use dioxus::prelude::*;
use im_rc::HashMap;

use crate::{item::index::Item, Todo};

#[component]
pub fn List(mut todos: Signal<HashMap<usize, Todo>>) -> Element {
    let todos_vec = todos.read().iter().map(|f| *f.0).collect::<Vec<_>>();

    rsx! {
        style { {include_str!("./index.css")} }
        ul { class: "todo-main",
            {todos_vec.iter().map(|id| rsx! {
                Item { key: "{id}", id: *id, set_todos: todos }
            })}
        }
    }
}
