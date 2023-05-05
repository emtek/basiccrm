use crate::{
    components::{nav_bar::Navbar, progress_bar::Progress},
    data::*,
    routes::AppRoute,
};

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncHandle, UseAsyncOptions};
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct CustomerDetailProps {
    pub id: CustomerId,
}

#[derive(Properties, PartialEq)]
pub struct SortArrowProps {
    pub pagination: CustomersQueryParams,
    pub field: CustomerSortField,
}

#[function_component(CustomersTable)]
pub fn customers_table() -> Html {
    let pagination = use_state(move || CustomersQueryParams {
        sort: CustomerSortField::Created,
        direction: SortDirection::Desc,
        offset: 0,
        limit: 20,
    });
    let query = pagination.clone();
    let customers: UseAsyncHandle<Vec<Customer>, _> = use_async_with_options(
        async move {
            let query_string = format!(
                "?sort={}&direction={}&offset={}&limit={}",
                query.sort, query.direction, query.offset, query.limit
            );
            get_data(format!("/customers{}", query_string)).await
        },
        UseAsyncOptions::enable_auto(),
    );
    let toggle_sort = |sort_by| {
        let current_page = pagination.clone();
        let customers_query = customers.clone();
        Callback::from(move |_| {
            current_page.set(CustomersQueryParams {
                sort: sort_by,
                direction: if current_page.direction == SortDirection::Asc {
                    SortDirection::Desc
                } else {
                    SortDirection::Asc
                },
                offset: 0,
                limit: current_page.limit.clone(),
            });
            customers_query.update(vec![Customer::default()]);
            customers_query.run();
        })
    };
    html! {
        <>
        <section class="hero is-primary">
            <Navbar/>
            <div class="hero-body">
                <p class="title">
                {"Customers"}
                </p>
            </div>
        </section>
        <section class="section">
            <table class="table is-fullwidth">
            <thead>
            <tr>
                <td onclick={toggle_sort(CustomerSortField::Name)}>{"Name"} <SortArrow pagination={*pagination} field={CustomerSortField::Name}/></td>
                <td onclick={toggle_sort(CustomerSortField::Email)}>{"Email"} <SortArrow pagination={*pagination} field={CustomerSortField::Email}/></td>
                <td onclick={toggle_sort(CustomerSortField::Status)}>{"Status"}<SortArrow pagination={*pagination} field={CustomerSortField::Status}/></td>
            </tr>
            </thead>
            if let Some(customers) = customers.data.clone() {
                        <tbody>
                        {
                            customers.into_iter().map(|p|
                                html!{
                                <tr>
                                    <td>
                                        <Link<AppRoute> to={AppRoute::CustomerDetail { id: p.id.clone() }}>
                                            {format!("{}", p.name)}</Link<AppRoute>>
                                    </td>
                                    <td>
                                        <Link<AppRoute> to={AppRoute::CustomerDetail { id: p.id.clone() }}>
                                            {format!("{}", p.email)}</Link<AppRoute>>
                                    </td>
                                    <td>
                                    {p.status}
                                    </td>
                                </tr>
                            }).collect::<Html>()
                        }
                        </tbody>
            } else {
                <Progress/>
            }
            </table>
        </section>
        </>
    }
}

#[function_component(SortArrow)]
pub fn sort_direction(props: &SortArrowProps) -> Html {
    if props.pagination.sort == props.field {
        match props.pagination.direction {
            SortDirection::Asc => {
                html! {<ion-icon class="" name="chevron-up"></ion-icon>}
            }
            SortDirection::Desc => {
                html! {<ion-icon class="" name="chevron-down"></ion-icon>}
            }
        }
    } else {
        html! {<></>}
    }
}
