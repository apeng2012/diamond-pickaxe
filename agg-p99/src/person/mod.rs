use dioxus::prelude::*;

#[component]
pub fn person() -> Element {
    let mut persons = use_context::<crate::PersonState>().persons;
    let mut name = use_signal(|| "".to_string());
    let mut age = use_signal(|| 0u8);

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
                oninput: move |e| age.set(e.value().parse().unwrap()),
            }
            button { onclick: move |_| persons.write().push((name(), age())), "Add" }
            ul {
                for (name , age) in persons.read().iter() {
                    li { {format!("{name} -- {age}")} }
                }
            }
        }
    }
}
