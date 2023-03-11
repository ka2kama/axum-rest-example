use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{extensions, EmptyMutation, EmptySubscription};
use async_graphql::{Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{response, Router};
use tracing::instrument::WithSubscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::module::Modules;

pub mod book_route;
mod graphql_route;
pub mod health_check_route;
pub mod user_route;

pub fn accumulate(
    Modules {
        user_usecase,
        book_usecase,
    }: Modules,
) -> Router {
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn)
                .with_resource(opentelemetry::sdk::Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", env!("CARGO_PKG_NAME")),
                    opentelemetry::KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                ]))
                .with_id_generator(opentelemetry::sdk::trace::RandomIdGenerator::default()),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .compact();

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .with(tracing_opentelemetry::layer().with_tracer(tracer.clone()))
        .init();
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(extensions::OpenTelemetry::new(tracer))
        .finish();
    Router::new()
        // .nest("/health", health_check_route::route())
        // .nest("/users", user_route::route(user_usecase))
        // .nest("/books", book_route::route(book_usecase))
        .route("/graphql", get(graphql_client).post(graphql_handler))
        .with_state(schema)
}

async fn graphql_client() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn graphql_handler(
    State(schema): State<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
pub struct QueryRoot;
#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        tracing::info!("hogehoge");
        "Hello World!"
    }
}
