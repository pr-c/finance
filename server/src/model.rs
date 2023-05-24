use crate::schema::books;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub name: String,
    pub bearer: Option<String>,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = books)]
pub struct Book {
    pub name: String,
    pub user_name: String,
}
