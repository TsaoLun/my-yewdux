use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store, Properties)]
pub struct YewduxStore {
    pub count: u32,
}

impl Default for YewduxStore {
    fn default() -> Self {
        Self { count: 5 }
    }
}

pub fn init() -> Dispatch<YewduxStore> {
    Dispatch::<YewduxStore>::new()  
}
