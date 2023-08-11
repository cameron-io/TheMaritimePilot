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
mod util;

/*
    Tokio Manages Non-Blocking Threads by Swapping Current Process between Core Thread & Blocking Thread

    Tokio is able to concurrently run many tasks on a few threads by repeatedly swapping the currently running task on each thread.
    - However, this kind of swapping can only happen at .await points
    - Thus, code that spends a long time without reaching an .await will prevent other tasks from running. 
    
    This is where the Core & blocking threads address this issue.
    The core threads are where all asynchronous code runs, and Tokio will by default spawn one for each CPU core.

    You can use the environment variable TOKIO_WORKER_THREADS to override the default value.
*/

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "maritime_pilot_postgres=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // initialize db connection & make migrations
    let db_pool = db::init().await;

    // build our application with some routes
    let app = Router::new()
        .route("/user/list", get(api::list_users))
        .route("/user/create", post(api::create_user))
        .with_state(db_pool);

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
