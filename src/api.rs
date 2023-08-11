use crate::schema::users;
use crate::db::User;
use crate::validate::NewUser;
use crate::util::internal_error;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;

use diesel::{SelectableHelper, RunQueryDsl, QueryDsl};
use deadpool_diesel::postgres::Pool;

pub async fn create_user(
    State(db_pool): State<Pool>,
    Json(new_user): Json<NewUser>
) -> Result<Json<User>, (StatusCode, String)> {
    let db_conn = db_pool.get().await.map_err(internal_error)?;
    let res = db_conn
        .interact(|db_conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(User::as_returning())
                .get_result(db_conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn list_users(
    State(db_pool): State<Pool>
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let db_conn = db_pool.get().await.map_err(internal_error)?;
    let res = db_conn
        .interact(|db_conn|
            users::table
                .select((users::id, users::username, users::email))
                .order_by(users::id)
                .load(db_conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}
