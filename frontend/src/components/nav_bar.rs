use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::*;
#[function_component(Navbar)]
pub fn nav_bar() -> Html {
    let state = use_state(move || NavState {
        hamburger_visible: false,
    });
    let visible = match state.hamburger_visible {
        true => Some("is-active"),
        false => None,
    };
    let toggle_hamburger = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(NavState {
                hamburger_visible: !state.hamburger_visible,
            })
        })
    };
    html! {
      <div class="hero-head">
      <nav class="navbar" role="navigation" aria-label="main navigation">
      <div class="container">
        <div class="navbar-brand">
          <Link<AppRoute> classes={"navbar-item"} to={AppRoute::CustomerList}>
            <img src="https://global-uploads.webflow.com/60ac1df26f2636ae4caabdbe/60ac1df26f26365ca1aac13a_Spider-icon-black.svg" loading="lazy" alt="spidertracks spider icon" class="spider-icon"/>
            <h2 style="margin-left:10px">{"BasicCRM"}</h2>
          </Link<AppRoute>>

          <a onclick={toggle_hamburger} role="button" class={classes!("navbar-burger",visible)} aria-label="menu" aria-expanded="false" data-target="navbarMenu">
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
          </a>
        </div>

        <div id="navbarMenu" class={classes!("navbar-menu",visible)}>
          <div class="navbar-end">
              <Link<AppRoute> classes={"navbar-item"} to={AppRoute::CustomerList}>{ "Home" }</Link<AppRoute>>
          </div>

        </div>
        </div>
      </nav>
    </div>

    }
}

struct NavState {
    hamburger_visible: bool,
}
