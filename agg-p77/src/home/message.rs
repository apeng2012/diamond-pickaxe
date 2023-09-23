use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MessageArr {
    id: String,
    title: String,
}

pub fn message(cx: Scope) -> Element {
    let nav = use_navigator(cx);
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
                messages.read().iter().map(|ma| rsx! {
                    one_msg {ma: ma.clone()}
                })
            }
        }
        hr {}
        Outlet::<Route> {}
        hr {}
        button {
            onclick: move |_| {
                nav.go_back();
            },
            "Go back"
        }
        button {
            onclick: move |_| {
                nav.go_forward();
            },
            "Go forward"
        }
    })
}

#[inline_props]
fn one_msg(cx: Scope, ma: MessageArr) -> Element {
    let nav = use_navigator(cx);

    cx.render(rsx! {
        li {
            Link {
                to: Route::Detail {
                    id: ma.id.clone(),
                    title: ma.title.clone(),
                },
                "{ma.title}"
            }
            button {
                onclick: move |_| {
                    nav.push(Route::Detail {
                        id: ma.id.clone(),
                        title: ma.title.clone(),
                    });
                },
                "push view"
            }
            button {
                onclick: move |_| {
                    nav.replace(Route::Detail {
                        id: ma.id.clone(),
                        title: ma.title.clone(),
                    });
                },
                "replace view"
            }
        }
    })
}
