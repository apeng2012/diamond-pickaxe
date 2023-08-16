#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        A {}
    })
}

fn A(cx: Scope) -> Element {
    let car_name = use_state(cx, || "奔驰".to_string());

    cx.render(rsx! {
        div {
            div { "我是A组件" },
            button {onclick: move |_| car_name.set("奥拓".to_string()), "换车" },
            B {
                car_name: (**car_name).clone(),
            },
        }
    })
}

#[derive(PartialEq, Props)]
struct BProps {
    car_name: String,
}

fn B(cx: Scope<BProps>) -> Element {
    cx.render(rsx! {
        div { "我是B组件，接受到的车是{cx.props.car_name}" }
    })
}
