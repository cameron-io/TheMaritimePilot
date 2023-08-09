use std::env;
use deadpool_diesel::postgres::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::{table, Selectable, Queryable};

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Nullable<Text>,
    }
}

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct User {
    id: i32,
    name: String,
    email: Option<String>,
}

pub async fn init() -> Pool {
    let db_url: String = env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // this embeds the migrations into the application binary
    // the migration path is relative to the `CARGO_MANIFEST_DIR`
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/migrations");

    // run the migrations on server startup
    let conn = pool.get().await.unwrap();
    
    conn.interact(
        |conn| conn.run_pending_migrations(MIGRATIONS)
                   .map(|_| ()))
        .await
        .unwrap()
        .unwrap();

    return pool;
}
