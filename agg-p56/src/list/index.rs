#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::item::index::Item;

pub fn List(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./index.css") }
        ul { class: "todo-main",
            Item {}
            Item {}
            Item {}
            Item {}
        }
    })
}
