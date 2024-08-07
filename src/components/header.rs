use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let (contnet, _) = create_signal("㗊");
    view! {
        <h1
            class="text-3xl"
        >微信 {contnet} 开</h1>
    }
}
