use dioxus::prelude::*;

pub fn about(cx: Scope) -> Element {
    cx.render(rsx! { h3 { "I am the content of About" } })
}
