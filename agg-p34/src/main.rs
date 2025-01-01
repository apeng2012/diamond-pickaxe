use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut passwd = use_signal(|| "".to_string());

    rsx! {
        form {
            onsubmit: move |_| {
                gloo_dialogs::alert(
                    &format!(
                        "你输入的用户名是：{name}，你输入的密码是：{passwd}",
                    ),
                )
            },
            "用户名"
            input { value: "{name}", onchange: move |new| name.set(new.value()) }
            "密码"
            input {
                value: "{passwd}",
                onchange: move |new| passwd.set(new.value()),
            }
            button { r#type: "submit", "登录" }
        }
    }
}
