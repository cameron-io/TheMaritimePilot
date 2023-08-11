use diesel::Insertable;
use serde::Deserialize;
use crate::db::users;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    name: String,
    email: Option<String>,
}
