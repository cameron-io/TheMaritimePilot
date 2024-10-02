extern crate dotenv;
use dotenv::dotenv;
use std::env;

use axum::{
    routing::{get, post},
    Router
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, fmt};

mod api;
mod schema;
mod db;
mod model;
mod validate;
mod util;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!(
                    "{}_postgres=debug",
                    env::var("SERVER_NAME").unwrap())
                    .into()
                )
        )
        .with(fmt::layer())
        .init();

    // initialize db connection & make migrations
    let db_pool = db::init().await;

    // routes
    let app = Router::new()
        .route("/user/list", get(api::list_users))
        .route("/user/create", post(api::create_user))
        .with_state(db_pool);

    // setup socket
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    tracing::debug!("listening on {}", addr);

    // run
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
