use std::rc::Rc;

use crate::{
    components::{
        nav_bar::Navbar,
        progress_bar::{PageProgress, Progress},
    },
    data::Opportunity,
    data::*,
};
use uuid::Uuid;
use validator::Validate;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncHandle, UseAsyncOptions};

use yewdux::prelude::use_store;
use yewdux_input::InputDispatch;

use super::error::ComponentError;

#[derive(Properties, PartialEq)]
pub struct CustomerDetailProps {
    pub id: CustomerId,
}

#[derive(Properties, PartialEq)]
pub struct EditOpportunityModalProps {
    pub opportunity: Opportunity,
    pub open: bool,
}

fn is_valid(field: &str, state: &Rc<Opportunity>) -> Option<String> {
    match validation_message(field, state) {
        Some(_) => Some("is-danger".to_string()),
        None => None,
    }
}

fn validation_message(field: &str, state: &Rc<Opportunity>) -> Option<String> {
    match state.validate() {
        Err(error) => {
            let field_messages = error
                .field_errors()
                .iter()
                .filter(|(f, _)| f.clone().cmp(&field.clone()).is_eq())
                .map(|(_, v)| {
                    format!(
                        "{}",
                        v.into_iter()
                            .filter(|f| f.message.is_some())
                            .map(|f| f.message.clone().unwrap().to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                })
                .collect::<Vec<String>>();
            let overall_messages = error
                .field_errors()
                .iter()
                .filter(|(f, _)| f.clone().contains(&"__all__"))
                .map(|(_, v)| {
                    format!(
                        "{}",
                        v.into_iter()
                            .filter(|f| f.code.contains(field))
                            .map(|f| f.message.clone().unwrap().to_string())
                            .collect::<Vec<String>>()
                            .join("")
                    )
                })
                .filter(|s| s.len() > 0)
                .collect::<Vec<String>>();
            let combined = [field_messages, overall_messages].concat();
            if combined.len() == 0 {
                None
            } else {
                Some(combined.join(","))
            }
        }
        Ok(_) => None,
    }
}

#[function_component(CustomerDetail)]
pub fn customer_detail(props: &CustomerDetailProps) -> Html {
    let id = props.id.clone();
    let customer: UseAsyncHandle<Customer, _> = use_async_with_options(
        async move { get_data(format!("/customer/{}", id)).await },
        UseAsyncOptions::enable_auto(),
    );
    html! {
        <>
        if let Some(customer) = customer.data.clone() {
            <section class="hero is-primary">
            <Navbar/>
                <div class="hero-body">
                    <p class="title">
                    {format!("{}",customer.name)}
                    </p>
                    <p class="sub-title">
                    {&customer.email}
                    </p>
                </div>
            </section>
            <section class="section">
                <CustomerOpportunitiesList id={customer.id}/>
            </section>
        } else {
            if customer.error.is_some() {
                <ComponentError/>
            }else{
                <PageProgress/>
            }
        }
        </>
    }
}

#[function_component(CustomerOpportunitiesList)]
pub fn customer_opportunities_list(props: &CustomerDetailProps) -> Html {
    let id = props.id.clone();
    let opportunities: UseAsyncHandle<Vec<Opportunity>, MultiError> = use_async_with_options(
        async move { get_data(format!("/customer/{}/opportunities", id)).await },
        UseAsyncOptions::enable_auto(),
    );
    let (selected_opportunity, dispatch) = use_store::<Opportunity>();
    let modal_open = use_state(|| false);

    let close_modal = {
        let open_handle = modal_open.clone();
        Callback::from(move |_| {
            open_handle.set(false);
        })
    };
    let update = |opportunity: Rc<Opportunity>| {
        let reload_list = opportunities.clone();
        let o: Rc<Opportunity> = opportunity.clone();
        let customer_id = id.clone();
        let current_modal_state = modal_open.clone();
        dispatch.reduce_mut_future_callback(move |_| {
            let op = o.clone();
            let modal = current_modal_state.clone();
            let reload = reload_list.clone();
            Box::pin(async move {
                if op.id.eq(&Uuid::default()) {
                    if let Ok(_) =
                        post_data(format!("/customer/{}/opportunities", customer_id), op).await
                    {
                        modal.set(false);
                        reload.run();
                    }
                } else {
                    if let Ok(_) = put_data(
                        format!("/customer/{}/opportunity/{}", customer_id, op.id),
                        op,
                    )
                    .await
                    {
                        modal.set(false);
                        reload.run();
                    }
                }
                ()
            })
        })
    };
    let delete_opportunity = |opportunity_id: OpportunityId| {
        let reload_list = opportunities.clone();
        let selected_opportunity = opportunity_id.clone();
        let customer_id = id.clone();
        let current_modal_state = modal_open.clone();
        dispatch.reduce_mut_future_callback(move |_| {
            let modal = current_modal_state.clone();
            let reload = reload_list.clone();
            Box::pin(async move {
                if let Ok(_) = delete_data(format!(
                    "/customer/{}/opportunity/{}",
                    customer_id,
                    selected_opportunity.clone()
                ))
                .await
                {
                    modal.set(false);
                    reload.run();
                }
                ()
            })
        })
    };
    let selected_option = |status: OpportunityStatus| {
        format!("{}", status).eq(&dispatch.get().clone().status.clone())
    };
    let select_opportunity = |opportunity: Opportunity| {
        let current_modal_state = modal_open.clone();
        let new_opportunity = opportunity.clone();
        dispatch.reduce_mut_callback(move |state| {
            *state = new_opportunity.to_owned();
            current_modal_state.set(true);
        })
    };
    let add_opportunity = select_opportunity(Opportunity {
        status: format!("{}", OpportunityStatus::New),
        ..Opportunity::default()
    });
    fn modal_label(id: OpportunityId) -> String {
        match id.clone().eq(&Uuid::default()) {
            true => "Create".to_string(),
            false => "Edit".to_string(),
        }
    }
    fn modal_visible(open: bool) -> Option<String> {
        if open {
            Some("is-active".to_string())
        } else {
            None
        }
    }
    fn submit_disabled(state: &Rc<Opportunity>) -> bool {
        match state.validate() {
            Err(_) => true,
            Ok(_) => false,
        }
    }
    html! {
            <>
        if let Some(opportunities) = opportunities.data.clone() {
            <div class={classes!("modal",modal_visible(*modal_open))}>
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{modal_label(selected_opportunity.clone().id.clone())}</p>
                        <button onclick={&close_modal} class="delete" aria-label="close"></button>
                    </header>
                        <section class="modal-card-body">
                            <div class="field">
                                <label class="label">{"Name"}</label>
                                <div class="control">
                                <input value={dispatch.get().name.clone()} oninput={dispatch.input_mut(|selected_opportunity, text| selected_opportunity.name = text)} class={classes!("input",is_valid("name", &selected_opportunity))} type="text" placeholder="Name"/>
                                </div>
                                <p class="help is-danger">{validation_message("name", &selected_opportunity)}</p>
                            </div>

                            <div class="field">
                                <label class="label">{"Status"}</label>
                                <div class="control">
                                <div class="select is-fullwidth">
                                <select >
                                    <option onclick={dispatch.reduce_mut_callback(|state| state.status = format!("{}", OpportunityStatus::New))} selected={selected_option(OpportunityStatus::New)} text={format!("{}", OpportunityStatus::New)}>{"New"}</option>
                                    <option onclick={dispatch.reduce_mut_callback(|state| state.status = format!("{}", OpportunityStatus::ClosedWon))} selected={selected_option(OpportunityStatus::ClosedWon)} value={format!("{}", OpportunityStatus::ClosedWon)}>{"Closed Won"}</option>
                                    <option onclick={dispatch.reduce_mut_callback(|state| state.status = format!("{}", OpportunityStatus::ClosedLost))} selected={selected_option(OpportunityStatus::ClosedLost)} value={format!("{}", OpportunityStatus::ClosedLost)}>{"Closed Lost"}</option>
                                </select>
                                </div>
                                </div>
                                <p class="help is-danger"></p>
                            </div>
                        </section>
                    <footer class="modal-card-foot">
                        <button disabled={submit_disabled(&dispatch.get())} onclick={&update(dispatch.get())} class="button is-success">{"Save changes"}</button>
                        <button onclick={&close_modal} class="button">{"Cancel"}</button>
                    </footer>
                </div>
            </div>

            <div class="field is-grouped">
                <div class="control">
                    <button onclick={add_opportunity} class="button is-link">{"Add opportunity"}</button>
                </div>
            </div>
                <table class="table is-fullwidth">
                <thead>
                <tr>
                    <td>{"Name"}</td>
                    <td>{"Status"}</td>
                    <td>{""}</td>
                </tr>
                </thead>
                <tbody>
                {
                    opportunities.iter().map(|o| {
                        html!{
                        <tr>
                            <td>{&o.name}</td>
                            <td>{&o.status}</td>
                            <td>
                            <div class="field is-grouped">
                                <div class="control">
                                    <button onclick={select_opportunity(o.clone())}class="button is-info" ><ion-icon class="" name="pencil"/></button>
                                    <button onclick={delete_opportunity(o.id.clone())}class="button is-danger" ><ion-icon class="" name="trash"/></button>
                                </div>
                                </div>
                            </td>
                        </tr>
                        }
                    }).collect::<Html>()
                }
                </tbody>
            </table>
        } else {
            if opportunities.error.is_some() {
                <ComponentError />
            }else{
                <Progress/>
            }
        }
        </>
    }
}
