use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::YewduxStore;

#[function_component(Display)]
pub fn view() -> Html {
    let (state, _) = use_store::<YewduxStore>();
    let username = &state.username;
    let password = &state.password;
     html!{
        <div>
            <h1>{"Display Form"}</h1>
            <p>{format!("Username: {}, Password: {}", username, password)}</p>
        </div>
     }
}