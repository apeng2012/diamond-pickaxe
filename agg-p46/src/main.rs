use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let list_style = r"
        width: 200px;
        height: 150px;
        background: skyblue;
        overflow: auto;
    ";

    let news_style = r"
        height: 30px;
    ";

    let mut news = use_signal(Vec::<String>::new);

    use_future(move || async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
            let num = news.read().len();
            news.write().push(format!("新闻 {}", num + 1));
        }
    });

    rsx! {
        div { style: "{list_style}",
            {news.read().iter().map(|value| rsx! {
                div { style: "{news_style}", "{value}" }
            })}
        }
    }
}
