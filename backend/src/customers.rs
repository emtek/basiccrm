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
    use frontend::{CustomerSortField, SortDirection};

    use super::*;

    #[tokio::test]
    async fn customers_should_return_result() {
        let db = edgedb_tokio::create_client()
            .await
            .expect("Failed to connect to the DB");
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
}
