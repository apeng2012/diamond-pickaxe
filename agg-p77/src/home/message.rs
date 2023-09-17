use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone)]
pub struct MessageArr {
    id: String,
    title: String,
}

pub fn message(cx: Scope) -> Element {
    let messages = use_ref(cx, || {
        vec![
            MessageArr { id: "01".to_string(), title: "message1".to_string() },
            MessageArr { id: "02".to_string(), title: "message2".to_string() },
            MessageArr { id: "03".to_string(), title: "message3".to_string() },
        ]
    });
    cx.render(rsx! {
        div {
            ul {
                messages.read().iter().map(|msg| rsx! {
                    li { Link {to: Route::Detail { id: msg.id.clone(), title: msg.title.clone()}, "hehe"}} //"{msg.title}"}}
                })
            }
        }
        Outlet::<Route> {}
    })
}
