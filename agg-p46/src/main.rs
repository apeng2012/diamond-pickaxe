#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let list_style = r"
        width: 200px;
        height: 150px;
        background: skyblue;
        overflow: auto;
    ";

    let news_style = r"
        height: 30px;
    ";

    let news = use_ref(cx, Vec::<String>::new);

    use_future(cx, (news,), |(news,)| async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
            let num = news.read().len();
            news.write().push(format!("新闻 {}", num + 1));
        }
    });

    cx.render(rsx! {
        div { style: "{list_style}",
            news.read().iter().map(|value| rsx!(
                div { style: "{news_style}", "{value}" }
            ))
        }
    })
}
