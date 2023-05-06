use axum::{routing::get_service, Router};
use customers::customer_routes;
use edgedb_tokio::RetryOptions;
use opentelemetry::sdk::trace::{self};
use opentelemetry::{
    global::{self},
    sdk::{propagation::TraceContextPropagator, Resource},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use std::time::Duration;
use std::{collections::HashMap, env, net::SocketAddr, path::PathBuf};
use tokio::signal;
use tower_http::{catch_panic::CatchPanicLayer, services::ServeFile, trace::TraceLayer};
use tracing::instrument::WithSubscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter, Layer};
mod customers;

async fn setup_server() -> Router {
    let assets_dir = PathBuf::from("./dist");
    let static_files_service = get_service(
        tower_http::services::ServeDir::new(assets_dir)
            .append_index_html_on_directories(true)
            .fallback(ServeFile::new("./dist/index.html")),
    );

    let edge_db = edgedb_tokio::create_client()
        .await
        .expect("Failed to connect to the DB")
        .with_retry_options(RetryOptions::default().new(3, |attempt: u32| {
            Duration::from_millis(10 * attempt.pow(2) as u64)
        }));

    Router::new()
        .fallback(static_files_service)
        .nest("/api", customer_routes().with_state(edge_db))
        .layer(CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Shutdown signal received, starting graceful shutdown");
}

#[tokio::main]
async fn main() {
    // Setting a trace context propagation data.
    global::set_text_map_propagator(TraceContextPropagator::new());
    // initialize tracing output to stdout
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", tracing::Level::TRACE)
        .with_target("tower_http::trace::on_request", tracing::Level::TRACE)
        .with_target("tower_http::trace::on_failure", tracing::Level::ERROR)
        .with_target("hyper", tracing::Level::ERROR)
        .with_default(tracing::Level::INFO);
    let layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(filter.clone());

    if let Ok(api_key) = env::var("HONEYCOMB_API_KEY") {
        let exporter = opentelemetry_otlp::new_exporter()
            .http()
            .with_endpoint("https://api.honeycomb.io/v1/traces")
            .with_http_client(reqwest::Client::default())
            .with_headers(HashMap::from([("x-honeycomb-team".into(), api_key)]));
        let otlp_tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(
                trace::config().with_resource(Resource::new(vec![KeyValue::new(
                    opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                    "BasicCrm".to_string(),
                )])),
            )
            .install_batch(opentelemetry::runtime::Tokio)
            .expect("Error - Failed to create tracer.");

        tracing_subscriber::registry()
            .with(
                tracing_opentelemetry::layer()
                    .with_tracer(otlp_tracer)
                    .with_filter(filter),
            )
            .init();
    } else {
        // just print to console
        tracing_subscriber::registry().with(layer).init();
    }

    let router = setup_server().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("BasicCrm backend listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .with_current_subscriber()
        .await
        .expect("Server could not start");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_should_be_valid() {
        let _ = setup_server();
    }
}
