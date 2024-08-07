use leptos::*;

#[component]
pub fn Discription(#[prop(into)] content: String) -> impl IntoView {
    view! {

        <p
            class = "
                w-4/5
            "
        >{content}</p>
    }
}
