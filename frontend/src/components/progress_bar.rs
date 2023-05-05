use yew::prelude::*;

use crate::components::nav_bar::Navbar;

/// Progress for sub components
#[function_component(Progress)]
pub fn progress_indeterminate() -> Html {
    html! {
    <section class="section is-medium">
      <div class="container">
        <div class="columns is-vcentered">
          <div class="column has-text-centered">
            <progress class="progress is-info" max="100"></progress>
          </div>
        </div>
      </div>
    </section>
    }
}

/// Progress with the nav section
#[function_component(PageProgress)]
pub fn page_progress_indeterminate() -> Html {
    html! {
      <>
        <section class="hero is-primary">
          <Navbar/>
          <div class="hero-body">
          </div>
        </section>
        <section class="section">
            <Progress/>
        </section>
      </>
    }
}
