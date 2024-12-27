#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Person { name: "jerry", age: 19 }
        Person { name: "tom", sex: "女" }
        Person { name: "老刘", age: 30, sex: "女" }
    }
}

#[derive(PartialEq, Props, Clone)]
struct PersonProps {
    #[props(into)]
    name: String,
    #[props(default = 18)]
    age: u8,
    #[props(into)]
    sex: Option<String>,
}

fn Person(props: PersonProps) -> Element {
    rsx! {
        ul {
            li { "姓名: {props.name}" }
            li { "性别: {props.age}" }
            li {
                "年龄: "
                {props.sex.unwrap_or_else(|| "男".to_string())}
            }
        }
    }
}
