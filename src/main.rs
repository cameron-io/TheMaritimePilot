extern crate dotenv;
use dotenv::dotenv;

use axum::{
    routing::{get, post},
    Router
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod db;
mod validate;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "example_tokio_postgres=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // initialize db connection & make migrations
    let pool = db::init().await;

    // build our application with some routes
    let app = Router::new()
        .route("/user/list", get(api::list_users))
        .route("/user/create", post(api::create_user))
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
