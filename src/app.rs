use crate::{
    store::{init, YewduxStore},
    display::Display,
    login::Login,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn view() -> Html {
    html! {
        <div>
            <h1>{"App"}</h1>
            <Login />
            <Display />
        </div>
    }
}
