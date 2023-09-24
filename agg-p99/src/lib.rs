use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    render!(rsx! { my_count {}})
}

fn my_count(cx: Scope) -> Element {
    let cnt = use_state(cx, || 0);
    let select_number = use_state(cx, || 1);
    let delay1s_inc = use_future(cx, (cnt, select_number), |(cnt, select_number)| async move {
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        *cnt + *select_number
    });

    render!(rsx! {
        h1 { "click {**cnt} times"}
        select {
            oninput: move |evt| {select_number.set(evt.value.parse::<i32>().unwrap_or_default())},
            option {
                value: "1",
                label: "1",
            }
            option {
                value: "2",
                label: "2",
            }
            option {
                value: "3",
                label: "3",
            }
        }
        button {
            onclick: move |_| {
                cnt.set(**cnt + **select_number);
            },
            "+"
        }
        button {
            onclick: move |_| {
                cnt.set(**cnt - **select_number);
            },
            "-"
        }
        button {
            onclick: move |_| {
                if **cnt % 2 != 0 {
                    cnt.set(**cnt + **select_number);
                }
            },
            "increment if odd"
        }
        button {
            onclick: move |_| {
                cnt.set(*delay1s_inc.value().unwrap());
            },
            "increment async"
        }
    })
}
