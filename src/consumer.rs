use yew::prelude::*;

#[function_component(Consumer)]
pub fn consumer() -> Html {
    html! {
        <div class={"consumer"}>
            <h3>{"Consumer"}</h3>
            <p>{"This is a simple Yew app."}</p>
        </div>
    }
}