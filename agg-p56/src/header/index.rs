use dioxus::html::input_data::keyboard_types;
use dioxus::prelude::*;

use crate::{Todo, TodoProps};

#[component]
pub fn Header(mut props: TodoProps) -> Element {
    let mut todo_name = use_signal(String::new);

    rsx! {
        style { {include_str!("./index.css")} }
        div { class: "todo-header",
            input {
                "type": "text",
                value: "{todo_name}",
                placeholder: "Please enter your task name and press Enter to confirm.",
                oninput: move |event| {
                    todo_name.set(event.value());
                },
                onkeyup: move |event| {
                    if event.data.code() != keyboard_types::Code::Enter {
                        return;
                    }
                    if todo_name().trim() == "" {
                        gloo_dialogs::alert("Input cannot be empty!");
                        return;
                    }
                    let id = props.todos.len() + 1;
                    props
                        .todos
                        .insert(
                            id,
                            Todo {
                                name: todo_name(),
                                done: false,
                            },
                        );
                    todo_name.set("".to_string());
                },
            }
        }
    }
}
