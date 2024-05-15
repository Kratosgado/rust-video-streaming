mod constants;
mod consumer;
mod context;
mod producer;
mod utils;

use consumer::Consumer;
use context::VideoChunksProvider;
use producer::Producer;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    log::info!("starting app");
    html! {
    <VideoChunksProvider>
        <div class={"grid"}>
            <Producer />
            <Consumer />
        </div>
    </VideoChunksProvider>
    }
}

#[wasm_bindgen(main)]
fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    yew::Renderer::<App>::new().render();
}
