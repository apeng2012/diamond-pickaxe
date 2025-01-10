use dioxus::prelude::*;

use crate::TodoProps;

#[component]
pub fn Footer(mut props: TodoProps) -> Element {
    let done_cnt = props
        .todos
        .values()
        .fold(0, |acc, todo| acc + if todo.done { 1 } else { 0 });
    let total = props.todos.len();
    let all_checked = props.todos.values().all(|todo| todo.done);
    let all_checked = all_checked && total != 0;
    let done_key_list = props
        .todos
        .iter()
        .filter(|(_, todo)| todo.done)
        .map(|(key, _)| *key)
        .collect::<Vec<_>>();
    let mut props_clone = props.clone();

    rsx! {
        style { {include_str!("./index.css")} }
        div { class: "todo-footer",
            label {
                input {
                    "type": "checkbox",
                    checked: "{all_checked}",
                    oninput: move |event| {
                        for (_, todo) in props_clone.todos.iter_mut() {
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
                            props.todos.remove(key);
                        })
                },
                "Clear Completed Tasks"
            }
        }
    }
}
