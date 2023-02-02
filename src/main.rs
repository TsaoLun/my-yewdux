mod app;
mod store;
mod display;
mod login;
use app::App;
fn main() {
    yew::Renderer::<App>::new().render();
}

