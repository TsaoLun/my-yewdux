use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Default, PartialEq, Store, Properties)]
pub struct YewduxStore {
    pub count: u32,
}

pub fn init() -> Dispatch<YewduxStore> {
    Dispatch::<YewduxStore>::new()
}
