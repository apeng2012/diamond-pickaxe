use crate::Route;
use dioxus::prelude::*;

pub fn news() -> Element {
    let is_timeout = use_signal(|| false);
    // use_future(cx, (is_timeout,), |(is_timeout,)| async move {
    //     async_std::task::sleep(std::time::Duration::from_secs(3)).await;
    //     is_timeout.set(true);
    // });

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
