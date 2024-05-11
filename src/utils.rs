use std::rc::Rc;

use web_sys::EncodedVideoChunk;
use yew::Reducible;


#[derive(Clone, PartialEq)]
pub struct EncodedVideoChunkWrapper {
    pub chunk: Option<EncodedVideoChunk>
}

impl Reducible for EncodedVideoChunkWrapper {
    type Action = EncodedVideoChunkWrapper;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.clone().into()
    }
}
