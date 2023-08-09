#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Person { name: "jerry", age: 19, sex: "男"},
        Person { name: "tom", age: 18, sex: "女"},
        Person { name: "老刘", age: 30, sex: "女"},
    })
}

#[derive(Props)]
struct PersonProps<'a> {
    name: &'a str,
    age: u8,
    sex: &'a str,
}

fn Person<'a>(cx: Scope<'a, PersonProps<'a>>) -> Element {
    cx.render(rsx! {
        ul {
            li { "姓名: {cx.props.name}" }
            li { "性别: {cx.props.age}" }
            li { "年龄: {cx.props.sex}" }
        }
    })
}
