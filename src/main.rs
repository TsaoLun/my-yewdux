mod app;
mod counter;
mod store;
use app::App;
fn main() {
    yew::Renderer::<App>::new().render();
}

