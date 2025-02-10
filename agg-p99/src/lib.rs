use dioxus::prelude::*;

//mod person;

static CNT: GlobalSignal<i32> = Global::new(|| 0);

#[component]
pub fn app() -> Element {
    rsx! {
        my_count {}
        hr {}
    }
}

#[component]
fn my_count() -> Element {
    let mut select_number = use_signal(|| 1);
    let mut delay1s_inc = use_resource(move || async move {
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        *CNT.write() += select_number();
    });

    rsx! {
        h2 { "I am a count component" }
        h4 { "click {CNT} times" }
        select { oninput: move |evt| { select_number.set(evt.value().parse::<i32>().unwrap_or_default()) },
            option { value: "1", label: "1" }
            option { value: "2", label: "2" }
            option { value: "3", label: "3" }
        }
        button {
            onclick: move |_| {
                *CNT.write() += select_number();
            },
            "+"
        }
        button {
            onclick: move |_| {
                *CNT.write() -= select_number();
            },
            "-"
        }
        button {
            onclick: move |_| {
                if CNT() % 2 != 0 {
                    *CNT.write() += select_number();
                }
            },
            "increment if odd"
        }
        button {
            onclick: move |_| {
                delay1s_inc.restart();
            },
            "increment async"
        }
    }
}
