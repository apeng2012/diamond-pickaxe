use crate::Route;
use dioxus::prelude::*;

pub fn news() -> Element {
    let mut is_timeout = use_signal(|| false);
    let _ = use_resource(move || async move {
        async_std::task::sleep(std::time::Duration::from_secs(3)).await;
        is_timeout.set(true);
    });

    if is_timeout() {
        let nav = navigator();
        nav.push(Route::Detail {
            id: "01".to_string(),
            title: "message1".to_string(),
        });
    }

    rsx! {
        ul {
            li { "news001" }
            li { "news002" }
            li { "news003" }
        }
    }
}
