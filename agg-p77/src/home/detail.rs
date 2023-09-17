use dioxus::prelude::*;

struct Content {
    id: String,
    content: String,
}

#[inline_props]
pub fn detail(cx: Scope, id: String, title: String) -> Element {
    let contents = vec![
        Content { id: "01".to_string(), content: "Hi, China".to_string() },
        Content { id: "02".to_string(), content: "Hi, Hebei".to_string() },
        Content { id: "03".to_string(), content: "Hi, Baoding".to_string() },
    ];

    let find_content = &contents
        .iter()
        .find(|c| c.id == *id)
        .unwrap_or(&contents[0])
        .content;

    cx.render(rsx! {
        ul {
            li { "ID: {*id}" }
            li { "TITLE: {*title}" }
            li { "CONTENT: {*find_content}" }
        }
    })
}
