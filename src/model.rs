use diesel::{Selectable, Queryable};
use crate::schema::users;

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct User {
    id: i32,
    username: String,
    email: Option<String>,
}
