use std::collections::HashMap;

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut values = use_signal(HashMap::new);

    rsx! {
        form {
            onsubmit: move |event| {
                values.set(event.values());
                values
                    .with_mut(|x| {
                        gloo_dialogs::alert(
                            &format!(
                                "你输入的用户名是：{}，你输入的密码是：{}",
                                x.get("name").unwrap().as_value(),
                                x.get("passwd").unwrap().as_value(),
                            ),
                        )
                    });
            },
            "用户名"
            input { name: "name" }
            "密码"
            input { name: "passwd" }
            button { r#type: "submit", "登录" }
        }
    }
}
