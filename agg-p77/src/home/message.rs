use dioxus::prelude::*;

pub fn message(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            ul {
                li { a { href: "/message1", "message1" } }
                li { a { href: "/message2", "message2" } }
                li { a { href: "/message3", "message3" } }
            }
        }
    })
}
