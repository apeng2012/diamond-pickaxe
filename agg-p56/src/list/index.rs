#![allow(non_snake_case)]
use dioxus::prelude::*;
use im_rc::HashMap;

use crate::item::index::Item;
use crate::Todo;

#[inline_props]
pub fn List<'a>(cx: Scope, todos: &'a UseRef<HashMap<i32, Todo>>) -> Element {
    let todos_vec = todos.read().iter().map(|f| *f.0).collect::<Vec<_>>();

    cx.render(rsx! {
        style { include_str!("./index.css") }
        ul { class: "todo-main",
            todos_vec.iter().map(|id| rsx!(Item {key: "{id}", id: *id, set_todos: todos}))
        }
    })
}
