use crate::components::customer_detail::CustomerDetail;
use crate::components::{customers::CustomersTable, not_found::NotFound};
use crate::data::CustomerId;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum AppRoute {
    #[at("/")]
    CustomerList,
    #[at("/customer/:id")]
    CustomerDetail { id: CustomerId },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::CustomerDetail { id } => html! { <CustomerDetail id={id}/> },
        AppRoute::CustomerList => html! { <CustomersTable/> },
        AppRoute::NotFound => html! { <NotFound/> },
    }
}
