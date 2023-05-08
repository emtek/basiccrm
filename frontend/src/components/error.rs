use yew::prelude::*;

#[function_component(ComponentError)]
pub fn component_error() -> Html {
    html! {
      <>
        <section class="section is-medium">
          <div class="container">
            <div class="columns is-vcentered">
              <div class="column has-text-centered">
                <h1 class="title is-size-1">
                <ion-icon name="alert-circle-outline"></ion-icon>
                </h1>
                <h1 class="title">
                  {"Woops there was an error"}
                </h1>
                <p class="subtitle">{"Please try reloading the page"}</p>
              </div>
            </div>
          </div>
        </section>
    </>
    }
}
