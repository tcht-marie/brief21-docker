pub mod interfaces;
pub mod infrastructure;
pub mod application;
pub mod domain;
pub mod controllers;

use domain::repositories::UserRepository;

#[derive(Clone)]
pub struct AppState {
    user_repository: UserRepository
}

impl AppState {
    pub fn new(user_repository: UserRepository) -> Self {
        Self {
            user_repository
        }
    }
}
