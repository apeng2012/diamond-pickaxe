use crate::GithubUser;
use dioxus::prelude::*;

#[inline_props]
pub fn head_list<'a>(cx: Scope, users: &'a UseRef<Vec<GithubUser>>) -> Element {
    cx.render(rsx! {
        style { include_str!("./index.css") }
        div { class: "row",
            users.read().iter().map(|user| rsx! {
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
        }
    })
}
