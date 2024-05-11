mod constants;
mod consumer;
mod producer;
mod utils;
mod context;

use web_sys::{
    *,
};
use yew::prelude::*;
use producer::Producer;
use consumer::Consumer;

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
