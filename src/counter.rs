use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::YewduxStore;

pub struct Counter;

impl Component for Counter {
    type Message = ();
    type Properties = YewduxStore;
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let count = ctx.props().count;
        html! {
            <div>
                <h1>{"Counter"}</h1>
                <p>{format!("The button has been pressed {} times", count)}</p>
                <button>{"Click Me"}</button>
            </div>
        }
    }
}
