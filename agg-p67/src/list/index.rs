use crate::HeadState;
use dioxus::prelude::*;

#[inline_props]
pub fn head_list<'a>(cx: Scope, state: &'a UseRef<HeadState>) -> Element {
    cx.render(rsx! {
        style { include_str!("./index.css") }
        div { class: "row",
            match *state.read() {
                HeadState::First => rsx! { h2 { "Welcome to use, enter keywords, then click search" } },
                HeadState::Loading => rsx! { h2 { "Loading......" } },
                HeadState::Loaded(ref users) => rsx! {
                    users.iter().map(|user| rsx! {
                        div { key: "{user.id}", class: "card",
                            a { href: "{user.html_url}", target: "_blank",
                                img {
                                    src: "{user.avatar_url}",
                                    style: "width: 100px"
                                }
                            }
                            p { class: "card-text", "{user.login}" }
                        }
                    })
                },
                HeadState::Error(ref err) => rsx! { h2 { style: "color: red", "{err}" } },
            }
        }
    })
}
