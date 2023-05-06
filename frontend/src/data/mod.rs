use core::fmt;

use edgedb_derive::Queryable;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};
use yew::Properties;
use yewdux::store::Store;

pub type CustomerId = Uuid;
pub type OpportunityId = Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Asc,
    Desc,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SortDirection::Asc => write!(f, "asc"),
            SortDirection::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CustomerSortField {
    Name,
    Email,
    Status,
    Created,
}

impl fmt::Display for CustomerSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CustomerSortField::Name => write!(f, "name"),
            CustomerSortField::Email => write!(f, "email"),
            CustomerSortField::Status => write!(f, "status"),
            CustomerSortField::Created => write!(f, "created"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OpportunityStatus {
    New,
    ClosedWon,
    ClosedLost,
}

impl fmt::Display for OpportunityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OpportunityStatus::New => write!(f, "New"),
            OpportunityStatus::ClosedWon => write!(f, "ClosedWon"),
            OpportunityStatus::ClosedLost => write!(f, "ClosedLost"),
        }
    }
}

#[derive(Properties, Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct CustomersQueryParams {
    pub sort: CustomerSortField,
    pub direction: SortDirection,
    pub offset: usize,
    pub limit: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Validate)]
#[edgedb(json)]
pub struct Customer {
    pub id: CustomerId,
    #[validate(length(min = 3, max = 300, message = "Must be longer than 3 characters"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub status: String,
    pub created: String,
}

fn valid_opportunity_status(status: &str) -> Result<(), ValidationError> {
    match status {
        "New" => Ok(()),
        "ClosedWon" => Ok(()),
        "ClosedLost" => Ok(()),
        _ => Err(ValidationError {
            message: Some("Please enter a valid status".into()),
            ..ValidationError::new("status")
        }),
    }
}

#[derive(
    Properties, Default, Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Validate, Store,
)]
#[edgedb(json)]
pub struct Opportunity {
    pub id: OpportunityId,
    #[validate(length(min = 3, max = 300, message = "Must be longer than 3 characters"))]
    pub name: String,
    #[validate(custom = "valid_opportunity_status")]
    pub status: String,
    pub created: String,
}

pub async fn get_data<T>(path: String) -> Result<T, MultiError>
where
    T: serde::de::DeserializeOwned,
{
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}{}", get_base_url(), path))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await;
    match response {
        Err(_) => Err(MultiError::RequestError),
        Ok(response) => match response.text().await {
            Err(_) => Err(MultiError::RequestError),
            Ok(text) => match serde_json::from_str::<T>(&text) {
                Err(_) => Err(MultiError::DeserializeError),
                Ok(result) => Ok(result),
            },
        },
    }
}

pub async fn post_data<T>(path: String, body: T) -> Result<bool, MultiError>
where
    T: serde::ser::Serialize,
{
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}{}", get_base_url(), path))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .body(serde_json::to_string(&body).unwrap_or_default())
        .send()
        .await;
    match response {
        Err(_) => Err(MultiError::RequestError),
        Ok(_) => Ok(true),
    }
}

pub async fn put_data<T>(path: String, body: T) -> Result<bool, MultiError>
where
    T: serde::ser::Serialize,
{
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{}{}", get_base_url(), path))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .body(serde_json::to_string(&body).unwrap_or_default())
        .send()
        .await;
    match response {
        Err(_) => Err(MultiError::RequestError),
        Ok(_) => Ok(true),
    }
}

pub async fn delete_data(path: String) -> Result<bool, MultiError> {
    let client = reqwest::Client::new();
    let response = client
        .delete(format!("{}{}", get_base_url(), path))
        .send()
        .await;
    match response {
        Err(_) => Err(MultiError::RequestError),
        Ok(_) => Ok(true),
    }
}

fn get_base_url() -> String {
    if let Some(window) = web_sys::window() {
        match window.origin().contains("127") {
            true => format!("http://127.0.0.1:8080/api"), //fallback
            false => format!("{}{}", window.origin(), "/api"),
        }
    } else {
        format!("http://127.0.0.1:8080/api") //fallback
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MultiError {
    RequestError,
    DeserializeError, // etc.
}
