use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        A {}
    }
}

#[component]
fn A() -> Element {
    let mut car_name = use_signal(|| "奔驰".to_string());

    rsx! {
        div {
            div { "我是A组件" }
            button { onclick: move |_| car_name.set("奥拓".to_string()), "换车" }
            B { car_name: car_name() }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct BProps {
    #[props(into)]
    car_name: String,
}

#[component]
fn B(props: BProps) -> Element {
    rsx! {
        div { "我是B组件，接受到的车是{props.car_name}" }
    }
}
