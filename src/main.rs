mod constants;
mod consumer;
mod context;
mod producer;
mod utils;

use consumer::Consumer;
use context::VideoChunksProvider;
use producer::Producer;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <VideoChunksProvider>
        <div class={"grid"}>
            <Producer />
            <Consumer />
        </div>
    </VideoChunksProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
