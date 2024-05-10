use yew::prelude::*;

#[function_component(Producer)]
fn producer() -> Html {
    html! {
        <div class="producer">
            <h1>{"Producer"}</h1>
            <p>{"This is a simple Yew app."}</p>
        </div>
    }
}

#[function_component(Consumer)]
fn consumer() -> Html {
    html! {
        <div class={"consumer"}>
            <h1>{"Consumer"}</h1>
            <p>{"This is a simple Yew app."}</p>
        </div>
    }
}

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
