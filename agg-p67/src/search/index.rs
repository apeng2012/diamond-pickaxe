use crate::{GighubUsers, HeadState};
use dioxus::prelude::*;

#[component]
pub fn search(mut state: Signal<HeadState>) -> Element {
    let mut keyword = use_signal(String::new);
    let head = use_resource(move || async move {
        let url = format!("https://api.github.com/search/users?q={}", keyword);
        reqwest::get(&url).await?.json::<GighubUsers>().await
    });

    rsx! {
        style { {include_str!("./../css/bootstrap.css")} }
        section { class: "jumbotron",
            h3 { class: "jumbotron-heading", "Search Github Users" }
            div {
                input {
                    "type": "text",
                    value: "{keyword.read()}",
                    placeholder: "enter the name you search",
                    oninput: move |e| {
                        keyword.set(e.value());
                    },
                }
                button {
                    onclick: move |_| {
                        state
                            .set(
                                match head.read_unchecked().as_ref() {
                                    Some(Ok(github_users)) => {
                                        HeadState::Loaded(github_users.items.clone())
                                    }
                                    Some(Err(err)) => HeadState::Error(err.to_string()),
                                    None => HeadState::Loading,
                                },
                            );
                    },
                    "Search"
                }
            }
        }
    }
}
