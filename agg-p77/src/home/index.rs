use dioxus::prelude::*;

pub fn home(cx: Scope) -> Element {
    cx.render(rsx! { h3 { "I am the content of Home" } })
}
