use std::time::Duration;

use axum::{
    extract::{self, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};
use edgedb_protocol::value::Value;
use edgedb_tokio::Client;
use frontend::{Customer, CustomerId, CustomersQueryParams, Opportunity, OpportunityId};
use validator::Validate;

pub fn customer_routes() -> Router<Client> {
    Router::new()
        .route("/customers", get(customers))
        .route("/customer/:id", get(customer))
        .route(
            "/customer/:id/opportunities",
            get(opportunities).post(add_opportunity),
        )
        .route(
            "/customer/:id/opportunity/:oid",
            put(update_opportunity).delete(delete_opportunity),
        )
}

async fn customers(
    State(db): State<Client>,
    Query(pagination): extract::Query<CustomersQueryParams>,
) -> Response {
    let query = format!(
        r#"select <json>Customer {{
            id,
            name,
            email,
            status,
            created
        }} order by Customer.{} {} offset {} limit {}"#,
        &pagination.sort, &pagination.direction, &pagination.offset, &pagination.limit
    );
    tracing::trace!("{:?}", pagination);
    let result: Vec<Customer> = db
        .query(query.as_str(), &())
        .await
        .expect("Failed to query");
    (Json(result)).into_response()
}

async fn customer(State(db): State<Client>, Path(id): extract::Path<CustomerId>) -> Response {
    let result: Customer = db
        .query_required_single(
            r#"
            select <json>Customer {
                id,
                name,
                email,
                status,
                created
            } filter Customer.id = <uuid>$0 limit 1"#,
            &(id,),
        )
        .await
        .expect("Failed to query");
    (Json(result)).into_response()
}

async fn add_opportunity(
    State(db): State<Client>,
    Path(id): extract::Path<CustomerId>,
    Json(body): extract::Json<Opportunity>,
) -> Response {
    match body.validate() {
        Ok(_) => {
            let _: Value = db
                .query_required_single(
                    r#"
                    update Customer filter Customer.id = <uuid>$0
                    set {
                        opportunities += (insert Opportunity { name := <str>$1, status := <str>$2})
                    };"#,
                    &(id, body.name, body.status),
                )
                .await
                .expect("Failed to add");
            (StatusCode::OK).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST).into_response(),
    }
}

async fn update_opportunity(
    State(db): State<Client>,
    Path((id, oid)): extract::Path<(CustomerId, OpportunityId)>,
    Json(body): extract::Json<Opportunity>,
) -> Response {
    if body.id.ne(&oid) {
        return (StatusCode::BAD_REQUEST).into_response();
    }
    match body.validate() {
        Ok(_) => {
            let _: Value = db
                .query_required_single(
                    r#"
                    update Opportunity filter Opportunity.customer.id = <uuid>$0 and Opportunity.id = <uuid>$1
                    set {
                        name := <str>$2,
                        status := <str>$3
                    };"#,
                    &(id, body.id, body.name, body.status),
                )
                .await
                .expect("Failed to update");
            (StatusCode::OK).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST).into_response(),
    }
}

async fn opportunities(State(db): State<Client>, Path(id): extract::Path<CustomerId>) -> Response {
    let result: Vec<Opportunity> = db
        .query(
            r#"
            select <json>Opportunity {
                id,
                name,
                status,
                created
            } filter Opportunity.customer.id = <uuid>$0
            order by Opportunity.created desc"#,
            &(id,),
        )
        .await
        .expect("Failed to query");
    (Json(result)).into_response()
}

async fn delete_opportunity(
    State(db): State<Client>,
    Path((id, oid)): extract::Path<(CustomerId, OpportunityId)>,
) -> Response {
    let _: Value = db
        .query_required_single(
            r#"
            delete Opportunity filter Opportunity.customer.id = <uuid>$0 and Opportunity.id = <uuid>$1"#,
            &(id,oid),
        )
        .await
        .expect("Failed to delete");
    (StatusCode::OK).into_response()
}

#[cfg(test)]
mod tests {
    use edgedb_tokio::Error;
    use frontend::{CustomerSortField, SortDirection};
    use rand::distributions::{Alphanumeric, DistString};
    use serde::de::DeserializeOwned;

    use super::*;
    const TEST_EMAIL_DOMAIN: &str = "@test_email.com";

    async fn add_customer(db: &Client, customer: Customer) -> Result<Customer, Error> {
        db.query_required_single(
            r#"
            select <json>(
            insert Customer {
                name := <str>$0,
                email := <str>$1,
                status := <str>$2,
            }) 
            {
                id,
                name,
                email,
                status,
                created
            };"#,
            &(customer.name, customer.email, customer.status),
        )
        .await
    }

    async fn remove_customer(db: &Client, customer_id: CustomerId) -> Result<Value, Error> {
        db.query_required_single(
            r#"
            delete Customer filter Customer.id = <uuid>$0;"#,
            &(customer_id,),
        )
        .await
    }

    async fn get_db() -> Client {
        edgedb_tokio::create_client()
            .await
            .expect("Failed to connect to the DB")
    }

    async fn into_type<T>(response: Response) -> T
    where
        T: DeserializeOwned,
    {
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        serde_json::from_slice::<T>(&body).unwrap()
    }

    #[tokio::test]
    async fn customers_should_return_result() {
        let db = get_db().await;
        let result = customers(
            State(db),
            Query(CustomersQueryParams {
                sort: CustomerSortField::Created,
                direction: SortDirection::Desc,
                offset: 0,
                limit: 2,
            }),
        )
        .await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn add_and_remove_valid_customer_should_succeed() {
        let db = get_db().await;
        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let customer = add_customer(
            &db,
            Customer {
                name: format!("Test {}", random_string),
                email: format!("{}{}", random_string, TEST_EMAIL_DOMAIN),
                status: "Active".to_string(),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to add");
        let remove = remove_customer(&db, customer.id).await;
        assert_eq!(true, remove.is_ok());
    }

    #[tokio::test]
    async fn add_invalid_customer_should_fail() {
        let db = get_db().await;
        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let customer = add_customer(
            &db,
            Customer {
                name: format!("Test {}", random_string),
                email: format!("{}{}", random_string, TEST_EMAIL_DOMAIN),
                status: "InvalidStatus".to_string(),
                ..Default::default()
            },
        )
        .await;
        println!("{:?}", customer);
        assert_eq!(true, customer.is_err());
    }

    #[tokio::test]
    async fn add_valid_opportunity_should_succeed() {
        let db = get_db().await;
        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let customer = add_customer(
            &db,
            Customer {
                name: format!("Test {}", random_string),
                email: format!("{}{}", random_string, TEST_EMAIL_DOMAIN),
                status: "Active".to_string(),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to add");
        let response = add_opportunity(
            State(db.clone()),
            Path(customer.id),
            Json(Opportunity {
                name: format!("Opportunity {}", random_string),
                status: "New".to_string(),
                ..Default::default()
            }),
        )
        .await;
        assert_eq!(StatusCode::OK, response.status());
        let added_opportunities = opportunities(State(db.clone()), Path(customer.id)).await;
        let _ = remove_customer(&db, customer.id).await;
        assert_eq!(StatusCode::OK, added_opportunities.status());
        let results = into_type::<Vec<Opportunity>>(added_opportunities).await;
        assert_eq!(1, results.len());
        assert_eq!(
            format!("Opportunity {}", random_string),
            results.first().unwrap().name
        );
    }

    #[tokio::test]
    async fn add_invalid_opportunity_should_fail() {
        let db = get_db().await;
        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let customer = add_customer(
            &db,
            Customer {
                name: format!("Test {}", random_string),
                email: format!("{}{}", random_string, TEST_EMAIL_DOMAIN),
                status: "Active".to_string(),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to add");
        let response = add_opportunity(
            State(db.clone()),
            Path(customer.id),
            Json(Opportunity {
                name: format!("Opportunity {}", random_string),
                status: "InvalidStatus".to_string(),
                ..Default::default()
            }),
        )
        .await;
        let _ = remove_customer(&db, customer.id).await;
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }
}
