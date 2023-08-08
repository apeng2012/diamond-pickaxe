use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let is_hot = use_state(cx, || false);

    cx.render(rsx! {
        div {
            h1 {
                onclick: move |_| is_hot.set(!**is_hot),
                "今天天气很" if **is_hot { "炎热" } else { "凉爽" }
            }
        }
    })
}
