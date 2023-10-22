use dioxus::prelude::*;

pub fn person(cx: Scope) -> Element {
    // let cnt = use_atom_state(cx, &CNT);
    let name = use_state(cx, || "".to_string());
    let age = use_state(cx, || "".to_string());

    render!(rsx! {
        div {
            h2 { "I am a person component"}
            input {
                placeholder: "Enter name",
                value: "{name}",
                oninput: move |e| name.set(e.value.clone())
            }
            input {
                placeholder: "Enter age",
                value: "{age}",
                oninput: move |e| name.set(e.value.clone())
            }
            button {
                "Add"
            }
            ul {
                li {"name1 -- age1"}
                li {"name2 -- age2"}
                li {"name3 -- age3"}
            }
        }
    })
}
