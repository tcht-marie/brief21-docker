use getset::{Getters, Setters};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Getters, Setters, Default, sqlx::FromRow)]
#[getset(get = "pub", set = "pub")]
pub struct User {
    id: Option<i32>,
    username: String,
    email: Option<String>,
    password: Option<String>
}

