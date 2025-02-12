use dioxus::prelude::*;

#[component]
pub fn person() -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut age = use_signal(|| "".to_string());

    rsx! {
        div {
            h2 { "I am a person component" }
            input {
                placeholder: "Enter name",
                value: "{name}",
                oninput: move |e| name.set(e.value()),
            }
            input {
                placeholder: "Enter age",
                value: "{age}",
                oninput: move |e| age.set(e.value()),
            }
            button { "Add" }
            ul {
                li { "name1 -- age1" }
                li { "name2 -- age2" }
                li { "name3 -- age3" }
            }
        }
    }
}
