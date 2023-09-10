use dioxus::prelude::*;

#[inline_props]
pub fn head_list(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./index.css") }
        div { class: "row",
            div { class: "card",
                a { href: "https://github.com/reactjs", target: "_blank",
                    img {
                        src: "https://gimg2.baidu.com/image_search/src=http%3A%2F%2Fc-ssl.duitang.com%2Fuploads%2Fitem%2F201804%2F25%2F20180425234521_jVf8U.thumb.1000_0.jpeg&refer=http%3A%2F%2Fc-ssl.duitang.com&app=2002&size=f9999,10000&q=a80&n=0&g=0n&fmt=auto?sec=1696905335&t=7e8889dbca1d019febcea871ded75c80",
                        style: "width: 100px"
                    }
                }
                p { class: "card-text", "reactjs" }
            }
        }
    })
}
