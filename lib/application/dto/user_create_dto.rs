use getset::*;
use serde::Deserialize;

use crate::domain::entities::User;

#[derive(Deserialize, Getters, Setters, Debug, Default)]
#[getset(get = "pub", set = "pub")]
pub struct UserCreateDto {
    username: String,
    email: Option<String>,
    password: Option<String>,
}


impl From<UserCreateDto> for User {
    fn from(dto: UserCreateDto) -> Self {
        User::default()
            .set_username(dto.username().clone())
            .set_email(dto.email().clone())
            .set_password(dto.password().clone())
        .clone()
    }
}