#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let input1 = use_state(cx, || "".to_string());
    let input2 = use_state(cx, || "".to_string());

    cx.render(rsx! {
        div {
            input {
                placeholder: "点击按钮提示数据",
                value: "{input1}",
                oninput: move |e| input1.set(e.value.clone()),
            }
            button {
                onclick: move |_| gloo_dialogs::alert(&format!("{}", input1)),
                "点击我提示左侧的数据",
            }
            input {
                placeholder: "失去焦点提示数据",
                value: "{input2}",
                oninput: move |e| input2.set(e.value.clone()),
                onfocusout: move |_| gloo_dialogs::alert(&format!("{}", input2)),
            }
        }
    })
}
