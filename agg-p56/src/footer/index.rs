use dioxus::prelude::*;
use im_rc::HashMap;

use crate::Todo;

#[component]
pub fn Footer(mut todos: Signal<HashMap<usize, Todo>>) -> Element {
    let done_cnt = todos()
        .values()
        .fold(0, |acc, todo| acc + if todo.done { 1 } else { 0 });
    let total = todos().len();
    let all_checked = todos().values().all(|todo| todo.done);
    let all_checked = all_checked && total != 0;
    let done_key_list = todos()
        .iter()
        .filter(|(_, todo)| todo.done)
        .map(|(key, _)| *key)
        .collect::<Vec<_>>();

    rsx! {
        style { {include_str!("./index.css")} }
        div { class: "todo-footer",
            label {
                input {
                    "type": "checkbox",
                    checked: "{all_checked}",
                    oninput: move |event| {
                        for (_, todo) in todos.write().iter_mut() {
                            todo.done = event.value().parse().unwrap();
                        }
                    },
                }
            }
            span {
                span { "Completed {done_cnt}" }
                "/ All {total}"
            }
            button {
                class: "btn btn-danger",
                onclick: move |_| {
                    done_key_list
                        .iter()
                        .for_each(|key| {
                            todos.write().remove(key);
                        })
                },
                "Clear Completed Tasks"
            }
        }
    }
}
