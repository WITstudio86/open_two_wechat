use leptos::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Deserialize, Serialize)]
struct DoitArgs {
    num: u8,
}

#[component]
pub fn Button(
    #[prop(into)] tip: String,
    n: (ReadSignal<u8>, WriteSignal<u8>),
    message: (ReadSignal<String>, WriteSignal<String>),
) -> impl IntoView {
    let (n, _) = n;
    let (tip, _) = create_signal(tip);
    let (message, set_message) = message;
    let doit = move |_| {
        spawn_local(async move {
            let args = DoitArgs { num: n.get() };
            let args = to_value(&args).unwrap();
            let result = invoke("doit", args).await.as_string().unwrap();
            set_message.set(result);
        })
    };

    view! {
        <div class="tooltip" data-tip={tip}>
            <button
                class="btn btn-primary"
                onclick="my_modal_1.showModal()"
            on:click=doit
            >创建 {n} 个微信进程</button>

            <dialog id="my_modal_1" class="modal">
                <div class="modal-box">
                <h3 class="text-lg font-bold">运行结束</h3>
                <p class="py-4">{message}</p>
                <div class="modal-action">
                    <form method="dialog">
                    <button class="btn">Close</button>
                    </form>
                </div>
                </div>
            </dialog>
        </div>
    }
}
