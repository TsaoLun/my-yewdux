use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::YewduxStore;

#[function_component(Login)]
pub fn view() -> Html {
    let (store, dispatch) = use_store::<YewduxStore>();
    let handle_form_submit = dispatch.reduce_mut_callback_with(|state, event: SubmitEvent| {
        event.prevent_default();
        let token = "1d2d323423dasd".to_owned();
        state.token = token;
        log!("submit!");
    });
    let handle_username_change = dispatch.reduce_mut_callback_with(|state, e: Event| {
        state.username = e.target_unchecked_into::<HtmlInputElement>().value();
    });
    let handle_password_change = dispatch.reduce_mut_callback_with(|state, e: Event| {
        state.password = e.target_unchecked_into::<HtmlInputElement>().value();
    });
    html! {
        <form onsubmit={handle_form_submit}>
            <h1>{"Login"}</h1>
            <div>
                <input type="text" placeholder="username" onchange={handle_username_change} />
            </div>
            <div>
                <input type="password" placeholder="password" onchange={handle_password_change} />
            </div>
            <div>
                <button>{"Log in"}</button>
            </div>
        </form>
    }
}
