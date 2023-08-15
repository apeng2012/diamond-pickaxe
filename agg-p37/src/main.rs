#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let is_suicide = use_state(cx, || false);

    if **is_suicide {
        return None;
    }

    let opacity = use_state(cx, || 1.0);
    use_future(cx, opacity, |opacity| async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_millis(200)).await;
            opacity.set(if *opacity <= 0.0 { 1.0 } else { *opacity - 0.1 });
        }
    });

    cx.render(rsx! {
        div {
            h2 { opacity: "{opacity}", "React学不会怎么办？" },
            button { onclick: move |_| is_suicide.set(true), "不活了" },
        }
    })
}
