use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let frameworks = vec!["Angular", "React", "Vue"];

    rsx! {
        div {
            h1 { "前端js框架列表" }
            ul {
                for framework in frameworks {
                    li { "{framework}" }
                }
            }
        }
    }
}
