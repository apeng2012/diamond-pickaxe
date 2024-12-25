use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_hot = use_signal(|| false);

    rsx! {
        div {
            h1 {
                onclick: move |_| {
                    let h = *is_hot.read();
                    is_hot.set(!h);
                },
                "今天天气很"
                if *is_hot.read() {
                    "炎热"
                } else {
                    "凉爽"
                }
            }
        }
    }
}
