#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::collections::HashMap;

use crate::item::index::Item;
use crate::Todo;

#[inline_props]
pub fn List<'a>(cx: Scope, todos: &'a UseRef<HashMap<i32, Todo>>) -> Element {
    cx.render(rsx! {
        style { include_str!("./index.css") }
        ul { class: "todo-main",
            todos.read().iter().map(|(key, todo)| rsx! {
                Item {_id: *key, todo: todo.clone() }
            })
        }
    })
}
