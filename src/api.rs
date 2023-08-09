use crate::db::{User, users};
use crate::validate::NewUser;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;

use diesel::{SelectableHelper, RunQueryDsl, QueryDsl};
use deadpool_diesel::postgres::Pool;

pub async fn create_user(
    State(pool): State<Pool>,
    Json(new_user): Json<NewUser>
) -> Result<Json<User>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn list_users(
    State(pool): State<Pool>
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn|
            users::table
                .select((users::id, users::name, users::email))
                .order_by(users::id)
                .load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

// Utility function for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}