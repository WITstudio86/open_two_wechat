use leptos::*;
use leptos_open_wechat::*;

#[component]
pub fn App() -> impl IntoView {
    let message = create_signal("Hello, world!".to_string());

    let signal = create_signal(2_u8);
    view! {
        <div
            class="
                h-screen 
                flex flex-col items-center justify-around
            "
        >
            <Header />
            <Discription content="选择数量之后点击运行开始创建微信进程 , 默认两个" />
            <InputNumber n=signal/>
            <Button
                tip="为了确保正常运行 , 在运行前请先退出所有已登录的微信"
                n = signal
                message = message
            />
            <Footer />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
