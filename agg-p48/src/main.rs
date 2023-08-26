#![allow(non_snake_case)]
use chrono::prelude::*;
use dioxus::prelude::*;
use std::collections::HashMap;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Demo1 {}
        hr {}
        hr {}
        hr {}
        Demo2 {}
    })
}

fn Demo1(cx: Scope) -> Element {
    let dt = use_state(cx, || "".to_string());

    use_future(cx, (dt,), |(dt,)| async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
            let local: DateTime<Local> = Local::now();
            dt.set(format!("{:?}", local));
        }
    });

    cx.render(rsx! {
        div {
            h1 { "hello" }
            input { "type": "text" }
            span {
                "现在是：{dt}"
                input { "type": "text" }
            }
        }
    })
}

struct Person {
    name: String,
    age: u8,
}

fn Demo2(cx: Scope) -> Element {
    let persons = use_ref(cx, || {
        HashMap::from([
            (
                1,
                Person {
                    name: "小张".to_string(),
                    age: 18,
                },
            ),
            (
                2,
                Person {
                    name: "小李".to_string(),
                    age: 19,
                },
            ),
        ])
    });

    cx.render(rsx! {
        div {
            h2 { "展示人员信息" }
            button {
                onclick: move |_| {
                    persons.write().insert(3, Person {name: "小王".to_string(), age: 20});
                },
                "添加一个小王"
            }
            h3 { "使用索引值作为key" }
            ul {
                persons.read().values().enumerate().map(|(index, p)| rsx! {
                    li {
                        key: "{index}",
                        "{p.name} --- {p.age}",
                        input { "type": "text" },
                    }
                })
            }
            hr {}
            h3 { "使用数据的唯一标识作为key" }
            ul {
                persons.read().iter().map(|(key, p)| rsx! {
                    li {
                        key: "{key}",
                        "{p.name} --- {p.age}",
                        input { "type": "text" },
                    }
                })
            }
        }
    })
}
