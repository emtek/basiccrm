use yew::prelude::*;

use crate::components::nav_bar::Navbar;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
      <>
        <section class="hero is-primary">
          <Navbar/>
          <div class="hero-body">
              <p class="title">
              {"Page Not Found"}
              </p>
          </div>
        </section>
        <section class="section is-medium">
          <div class="container">
            <div class="columns is-vcentered">
              <div class="column has-text-centered">
                <h1 class="title is-size-1">
                <ion-icon name="warning-outline"></ion-icon>
                </h1>
                <h1 class="title">
                  {"404 Page Not Found"}
                </h1>
                <p class="subtitle">{"Please choose a page from above"}</p>
              </div>
            </div>
          </div>
        </section>
    </>
    }
}
