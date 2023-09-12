use dioxus::prelude::*;
use serde::Deserialize;

mod list;
mod search;

pub fn app(cx: Scope) -> Element {
    let users = use_ref(cx, Vec::<GithubUser>::new);

    cx.render(rsx! {
        style { include_str!("./css/bootstrap.css") }
        div { class: "container",
            search::index::search { users: users }
            list::index::head_list { users: users }
        }
    })
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct GighubUsers {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<GithubUser>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct GithubUser {
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
