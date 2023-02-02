use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Login)]
pub fn view() -> Html {
    let handle_form_submit = {
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
        })
    };
    let handle_username_change = Callback::from(|event: Event| {});
    let handle_password_change = Callback::from(|event: Event| {});
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
