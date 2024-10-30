use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    let message: &str = "This page should not be rendered. Something went wrong :c";

    html! {
        <main class="container">
            <h1>{ &*message }</h1>
        </main>
    }
}
