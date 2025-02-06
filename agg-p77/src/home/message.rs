use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MessageArr {
    id: String,
    title: String,
}

pub fn message() -> Element {
    let nav = navigator();
    let messages = use_signal(|| {
        vec![
            MessageArr {
                id: "01".to_string(),
                title: "message1".to_string(),
            },
            MessageArr {
                id: "02".to_string(),
                title: "message2".to_string(),
            },
            MessageArr {
                id: "03".to_string(),
                title: "message3".to_string(),
            },
        ]
    });
    rsx! {
        div {
            ul {
                {messages.read().iter().map(|ma| rsx! {
                    one_msg { ma: ma.clone() }
                })}
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
    }
}

#[component]
fn one_msg(ma: MessageArr) -> Element {
    let nav = navigator();

    rsx! {
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
                // onclick: move |_| {
                //     nav.replace(Route::Detail {
                //         id: ma.id.clone(),
                //         title: ma.title.clone(),
                //     });
                // },
                "replace view"
            }
        }
    }
}
