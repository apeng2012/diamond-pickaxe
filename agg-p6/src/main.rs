use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 {"前端js框架列表"}
            ul {
                ["Angular", "React", "Vue"]
                    .into_iter()
                    .map(|f| rsx!(
                        li { "{f}" }
                    ))
            }
        }
    })
}
