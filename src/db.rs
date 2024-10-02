use std::env;
use deadpool_diesel::postgres::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::PgConnection;

pub async fn init() -> Pool {
    let db_url: String = env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let db_manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let db_pool = deadpool_diesel::postgres::Pool::builder(db_manager)
        .build()
        .unwrap();

    // this embeds the migrations into the application binary
    // the migration path is relative to the `CARGO_MANIFEST_DIR`
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    // run the migrations on server startup
    let db_conn = db_pool.get().await.unwrap();
    
    // migration must return Result<Vec<MigrationVersion<T>>
    let migrate_fun = |db_conn: &mut PgConnection|
        db_conn.run_pending_migrations(MIGRATIONS)
            .map(|_| ());

    db_conn.interact(migrate_fun)
        .await
        .unwrap() // migrate_fun
        .unwrap(); // interact

    return db_pool;
}
