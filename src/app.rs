use yew::prelude::*;
use yewdux::prelude::*;
use crate::{counter::Counter, store::{YewduxStore, init}};

#[derive(PartialEq, Default, Store)]
pub struct App {
    dispatch: Dispatch<YewduxStore>
}

impl Component for App {
    type Message = ();
    type Properties = YewduxStore;

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = init();
        Self { dispatch }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"App"}</h1>
                <Counter />
            </div>
        }
    }
}