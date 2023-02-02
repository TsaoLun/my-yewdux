use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Display)]
pub fn view() -> Html {
    let username = "";
    let password = "";
     html!{
        <div>
            <h1>{"Display Form"}</h1>
            <p>{format!("Username: {}, Password: {}", username, password)}</p>
        </div>
     }
}