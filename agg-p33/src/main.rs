#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        form {
            onsubmit: move |event| {
                let name = &event.values.get("name").unwrap()[0];
                let passwd = &event.values.get("passwd").unwrap()[0];
                gloo_dialogs::alert(&format!("你输入的用户名是：{name}，你输入的密码是：{passwd}"))
            },
            "用户名" input { name: "name", },
            "密码" input { name: "passwd", },
            button { r#type: "submit", "登录" },
        }
    })
}
