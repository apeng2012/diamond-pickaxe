use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut input1 = use_signal(|| "".to_string());
    let mut input2 = use_signal(|| "".to_string());

    rsx! {
        div {
            input {
                placeholder: "点击按钮提示数据",
                value: "{input1}",
                oninput: move |e| input1.set(e.value()),
            }
            button { onclick: move |_| gloo_dialogs::alert(&format!("{}", input1)),
                "点击我提示左侧的数据"
            }
            input {
                placeholder: "失去焦点提示数据",
                value: "{input2}",
                oninput: move |e| input2.set(e.value()),
                onfocusout: move |_| gloo_dialogs::alert(&format!("{}", input2)),
            }
        }
    }
}
