use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store, Properties)]
pub struct YewduxStore {
    pub username: String,
    pub password: String,
    pub token: String,
}

impl Default for YewduxStore {
    fn default() -> Self {
        Self { username: "".to_string(), password: "".to_string(), token:"".to_string() }
    }
}

pub fn init() -> Dispatch<YewduxStore> {
    Dispatch::<YewduxStore>::new()  
}
