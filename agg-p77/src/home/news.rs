use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn news(cx: Scope) -> Element {
    let is_timeout = use_state(cx, || false);
    use_future(cx, (is_timeout,), |(is_timeout,)| async move {
        async_std::task::sleep(std::time::Duration::from_secs(3)).await;
        is_timeout.set(true);
    });

    if *is_timeout.get() {
        let nav = use_navigator(cx);
        nav.push(Route::Detail {
            id: "01".to_string(),
            title: "message1".to_string(),
        });
    }

    cx.render(rsx! {
        ul {
            li { "news001" }
            li { "news002" }
            li { "news003" }
        }
    })
}
