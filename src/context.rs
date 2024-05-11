use yew::prelude::*;

use crate::utils::EncodedVideoChunkWrapper;

pub type VideoChunkContext = UseReducerHandle<EncodedVideoChunkWrapper>;

#[derive(Properties, Debug, PartialEq)]
pub struct VideoChunksProviderProps {
    #[prop_or_default]
    pub children: Children,
}
#[function_component(VideoChunksProvider)]
pub fn video_chunks_imp(props: &VideoChunksProviderProps) -> Html {
    let msg: VideoChunkContext = use_reducer(|| EncodedVideoChunkWrapper { chunk: None });

    html! {
        <ContextProvider<VideoChunkContext> context = {msg}>
            {props.children.clone()}
        </ContextProvider<VideoChunkContext>>
    }
}
