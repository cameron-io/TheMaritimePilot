extern crate dotenv;
use dotenv::dotenv;
use std::env;

use axum::{
    routing::{get, post},
    Router
};
use std::net::SocketAddr;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod db;

// this embeddes the migrations into the application binary
// the migration path is releative to the `CARGO_MANIFEST_DIR`
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/migrations");

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_url: String = format!("postgresql://{}:{}@{}:{}/{}",
        env::var("DB_USER").unwrap(),
        env::var("DB_PASS").unwrap(),
        env::var("DB_HOST").unwrap(),
        env::var("DB_PORT").unwrap(),
        env::var("DB_NAME").unwrap());

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();

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
