#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let name = use_state(cx, || "".to_string());
    let passwd = use_state(cx, || "".to_string());

    cx.render(rsx! {
        form {
            onsubmit: move |_| {
                gloo_dialogs::alert(&format!("你输入的用户名是：{name}，你输入的密码是：{passwd}"))
            },
            "用户名" input {
                value: "{name}",
                onchange: move |new| name.set(new.value.clone()),
            },
            "密码" input {
                value: "{passwd}",
                onchange: move |new| passwd.set(new.value.clone()),
            },
            button { r#type: "submit", "登录" },
        }
    })
}
