use leptos::*;

#[component]
pub fn InputNumber(#[prop(into)] n: (ReadSignal<u8>, WriteSignal<u8>)) -> impl IntoView {
    let (n, set_n) = n;
    view! {
        <input
            placeholder="输入期望的数量,但是并不一定可行"
            on:input=move |ev| {
                set_n.set(
                    event_target_value(&ev).parse::<u8>().unwrap_or_default()
                )
            }
            class="
                input input-bordered input-primary w-full max-w-xs
            "
            value={n}
        type="number"/>
    }
}
