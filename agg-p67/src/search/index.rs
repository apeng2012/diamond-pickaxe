use crate::{GighubUsers, GithubUser};
use dioxus::prelude::*;

#[inline_props]
pub fn search<'a>(cx: Scope, users: &'a UseRef<Vec<GithubUser>>) -> Element {
    let keyword = use_ref(cx, || "".to_string());
    let head = use_future(cx, (keyword,), |(keyword,)| {
        get_head(keyword.read().clone())
    });

    cx.render(rsx! {
        style { include_str!("./../css/bootstrap.css") }
        section { class: "jumbotron",
            h3 { class: "jumbotron-heading", "Search Github Users" }
            div {
                input {
                    "type": "text",
                    value: "{keyword.read()}",
                    placeholder: "enter the name you search",
                    oninput: move |e| {
                        *keyword.write() = e.value.clone();
                    }
                }
                button {
                    onclick: move |_| {
                        if let Some(Ok(github_users)) = head.value() {
                            *users.write() = github_users.items.clone();
                        }
                    },
                    "Search"
                }
            }
        }
    })
}

async fn get_head(keyword: String) -> Result<GighubUsers, reqwest::Error> {
    let url = format!("https://api.github.com/search/users?q={}", keyword);
    reqwest::get(&url).await?.json().await
}
