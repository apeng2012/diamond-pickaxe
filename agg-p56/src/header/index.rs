use dioxus::html::input_data::keyboard_types;
use dioxus::prelude::*;
use im_rc::HashMap;

use crate::Todo;

#[component]
pub fn Header(mut todos: Signal<HashMap<usize, Todo>>) -> Element {
    let mut todo_name = use_signal(String::new);
    let length = use_memo(move || todos().len());

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
                    let id = length() + 1;
                    todos
                        .write()
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
