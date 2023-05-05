use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod data;
mod routes;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <main>
            <Switch<routes::AppRoute> render={routes::switch} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
