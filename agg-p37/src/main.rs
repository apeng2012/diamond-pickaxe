use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_suicide = use_signal(|| false);

    let mut opacity = use_signal(|| 1.0);

    use_future(move || async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            opacity.set(if opacity() <= 0.0 { 1.0 } else { opacity() - 0.1 });
        }
    });

    rsx! {
        div {
            h2 { opacity: "{opacity}", "React学不会怎么办？" }
            button { onclick: move |_| is_suicide.set(true), "不活了" }
        }
    }
}
