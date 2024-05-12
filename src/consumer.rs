use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{js_sys::Reflect,*};
use yew::prelude::*;

use crate::{constants::VIDEO_CODEC, context::VideoChunkContext};

#[function_component(Consumer)]
pub fn consumer() -> Html {
    let video_ctx = use_context::<VideoChunkContext>();
    let video_decoder: UseStateHandle<Option<VideoDecoder>> = use_state(|| None);

    if (*video_decoder).is_none() {
        let error_video = Closure::wrap(Box::new(|e| {
            web_sys::console::log_1(&e);
        }) as Box<dyn FnMut(JsValue)>);

        let output = Closure::wrap(Box::new(|chunk: JsValue| {
            // let chunk = Box::new(original_chunk.cl);
            let video_chunk = chunk.clone().unchecked_into::<HtmlImageElement>();
            let width = Reflect::get(&chunk.clone(), &JsValue::from_str("width")).unwrap().as_f64().unwrap();
            let height = Reflect::get(&chunk.clone(), &JsValue::from_str("height")).unwrap().as_f64().unwrap();
            let render_canvas = window().unwrap()
                .document().unwrap()
                .get_element_by_id("render").unwrap()
                .unchecked_into::<HtmlCanvasElement>();
            render_canvas.set_width(width as u32);
            render_canvas.set_height(height as u32);
            let ctx = render_canvas.get_context("2d").unwrap().unwrap().unchecked_into::<CanvasRenderingContext2d>();
            ctx.draw_image_with_html_image_element(&video_chunk, 0.0, 0.0).unwrap();
            video_chunk.unchecked_into::<VideoFrame>().close();
        }) as Box<dyn FnMut(JsValue)>);

        let local_video_decoder = VideoDecoder::new(
            &VideoDecoderInit::new(error_video.as_ref().unchecked_ref(),output.as_ref().unchecked_ref())
        ).unwrap();
        local_video_decoder.configure(&VideoDecoderConfig::new(&VIDEO_CODEC));
        video_decoder.set(Some(local_video_decoder));
    }

    html! {
        <div class={"consumer"}>
            <h3>{"Consumer"}</h3>
            <p>{"This is a simple Yew app."}</p>
        </div>
    }
}