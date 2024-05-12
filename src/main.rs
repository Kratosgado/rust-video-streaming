mod constants;
mod consumer;
mod context;
mod producer;
mod utils;

use consumer::Consumer;
use producer::Producer;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class={"grid"}>
            <Producer />
            <Consumer />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
