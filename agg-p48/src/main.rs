#![allow(non_snake_case)]
use chrono::prelude::*;
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! { Demo1 {} })
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
