use diesel::prelude::*;

// normally part of your generated schema.rs file
table! {
    users (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    name: String,
    hair_color: Option<String>,
}
