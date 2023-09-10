use dioxus::prelude::*;

#[inline_props]
pub fn search(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./../css/bootstrap.css") }
        section { class: "jumbotron",
            h3 { class: "jumbotron-heading", "Search Github Users" }
            div {
                input { "type": "text", placeholder: "enter the name you search" }
                button { "Search" }
            }
        }
    })
}
