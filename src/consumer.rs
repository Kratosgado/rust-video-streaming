use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    js_sys::{Reflect, Uint8Array},
    *,
};
use yew::prelude::*;

use crate::{constants::VIDEO_CODEC, context::VideoChunkContext};
use web_sys::EncodedVideoChunk;

#[function_component(Consumer)]
pub fn consumer() -> Html {
    let video_ctx = use_context::<VideoChunkContext>().unwrap();
    let video_decoder: UseStateHandle<Option<VideoDecoder>> = use_state(|| None);

    if (*video_decoder).is_none() {
        let error_video = Closure::wrap(Box::new(|e: JsValue| {
            log::error!("on error: {}", &e.as_string().unwrap());
        }) as Box<dyn FnMut(JsValue)>);

        let output = Closure::wrap(Box::new(|chunk: JsValue| {
            // let chunk = Box::new(original_chunk.cl);
            let video_chunk = chunk.clone().unchecked_into::<VideoFrame>();
            log::info!("getting {:?}", video_chunk);
            let width = video_chunk.display_width();
            let height = video_chunk.display_height();

            let render_canvas = window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("render")
                .unwrap()
                .unchecked_into::<HtmlCanvasElement>();
            render_canvas.set_width(width);
            render_canvas.set_height(height);
            let ctx = render_canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .unchecked_into::<CanvasRenderingContext2d>();
            ctx.draw_image_with_video_frame(&video_chunk, 0.0, 0.0)
                .unwrap();
            video_chunk.unchecked_into::<VideoFrame>().close();
        }) as Box<dyn FnMut(JsValue)>);

        log::info!("before local video decoder");
        let local_video_decoder = VideoDecoder::new(&VideoDecoderInit::new(
            error_video.as_ref().unchecked_ref(),
            output.as_ref().unchecked_ref(),
        ))
        .unwrap();
        error_video.forget();
        output.forget();
        local_video_decoder.configure(&VideoDecoderConfig::new(&VIDEO_CODEC));
        video_decoder.set(Some(local_video_decoder));
    } else if (*video_ctx).chunk.is_some() {
        log::info!("chunk received");
        let chunk = (*video_ctx).chunk.as_ref().unwrap();
        let mut video_vector = vec![0u8; chunk.byte_length() as usize];
        let video_message = video_vector.as_mut();
        chunk.copy_to_with_u8_array(video_message);
        let decoder = (*video_decoder).to_owned().unwrap();
        let data = Uint8Array::from(video_message.as_ref());
        let encoded_video_chunk = EncodedVideoChunk::new(&EncodedVideoChunkInit::new(
            &data,
            chunk.timestamp(),
            chunk.type_(),
        ))
        .unwrap();
        log::info!("chunk: {:?}", chunk.clone());

        decoder.decode(&encoded_video_chunk);
    }

    html! {
        <div class={"consumer"}>
            <h3>{"Consumer"}</h3>
            <canvas id="render"></canvas>
        </div>
    }
}
