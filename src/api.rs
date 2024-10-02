use crate::schema::users;
use crate::model::User;
use crate::validate::NewUser;
use crate::util::internal_error;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;

use diesel::{PgConnection, SelectableHelper, RunQueryDsl, QueryDsl};
use deadpool_diesel::postgres::Pool;

pub async fn create_user(
    State(db_pool): State<Pool>,
    Json(new_user): Json<NewUser>
) -> Result<Json<User>, (StatusCode, String)> {
    let db_conn = db_pool.get().await.map_err(internal_error)?;

    let db_create_fun = |db_conn: &mut PgConnection| {
        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(db_conn)
    };

    let res = db_conn
        .interact(db_create_fun)
        .await
        .map_err(internal_error)?  // db_conn.interact
        .map_err(internal_error)?; // db_create_fun

    Ok(Json(res))
}

pub async fn list_users(
    State(db_pool): State<Pool>
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let db_conn = db_pool.get().await.map_err(internal_error)?;

    let db_list_fun = |db_conn: &mut PgConnection| {
        users::table
        .select( (users::id, users::username, users::email) )
        .order_by(users::id)
        .load(db_conn)
    };

    let res = db_conn
        .interact(db_list_fun)
        .await
        .map_err(internal_error)?  // db_conn.interact
        .map_err(internal_error)?; // db_list_fun

    Ok(Json(res))
}
