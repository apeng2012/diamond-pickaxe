#![allow(non_snake_case)]
use dioxus::html::input_data::keyboard_types;
use dioxus::prelude::*;
use std::collections::HashMap;

use crate::Todo;

#[inline_props]
pub fn Header<'a>(cx: Scope, todos: &'a UseRef<HashMap<i32, Todo>>) -> Element {
    let todo_name = use_state(cx, String::new);

    cx.render(rsx! {
        style { include_str!("./index.css") }
        div { class: "todo-header",
            input {
                "type": "text",
                value: "{todo_name}",
                placeholder: "Please enter your task name and press Enter to confirm.",
                oninput: move |event| {
                    todo_name.set(event.value.clone());
                },
                onkeyup: move |event| {
                    if event.data.code() != keyboard_types::Code::Enter {
                        return;
                    }
                    if todo_name.trim() == "" {
                        gloo_dialogs::alert("Input cannot be empty!");
                        return;
                    }
                    let id = todos.read().len() as i32 + 1;
                    todos
                        .write()
                        .insert(
                            id,
                            Todo {
                                name: (**todo_name).clone(),
                                done: false,
                            },
                        );
                    todo_name.set("".to_string());
                }
            }
        }
    })
}
