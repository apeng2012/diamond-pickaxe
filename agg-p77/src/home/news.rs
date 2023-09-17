use dioxus::prelude::*;

pub fn news(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            li { "news001" }
            li { "news002" }
            li { "news003" }
        }
    })
}
