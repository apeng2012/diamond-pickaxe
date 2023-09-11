use dioxus::prelude::*;
use serde::Deserialize;

#[inline_props]
pub fn search(cx: Scope) -> Element {
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
                button { onclick: move |_| {head.value();}, "Search" }
            }
        }
    })
}

async fn get_head(keyword: String) -> Result<GighubUsers, reqwest::Error> {
    let url = format!("https://api.github.com/search/users?q={}", keyword);
    reqwest::get(&url).await?.json().await
}

#[derive(Clone, PartialEq, Deserialize)]
struct GighubUsers {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<GithubUser>,
}

#[derive(Clone, PartialEq, Deserialize)]
struct GithubUser {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
    pub score: f64,
}
