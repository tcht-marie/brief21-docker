
use crate::{domain::entities::User, infrastructure::persistence::UserService};

#[derive(Clone)]
pub struct UserRepository {
    service: UserService,
}

impl UserRepository {
    pub fn new(service: UserService) -> Self {
        UserRepository { service }
    }

    pub async fn save(self, user: User) -> Result<User, sqlx::Error> {
        if let None = user.id() {
            self.service.save(user).await
        } else {
           self.service.update(user).await
        }
    }
    
    pub async fn save_password(self, user: User) -> Result<User, sqlx::Error> {
        tracing::debug!("{:?}", user.id());
        if let None = user.id() {
            self.service.save_password(user).await
        } else {
           self.service.update_password(user).await
        }
    }

    pub async fn find_by_id(self, user_id: i32) -> Result<User, sqlx::Error> {
        self.service.find_by_id(user_id).await
    }

    pub async fn find_all(self, page: i32, size: i32, order: String) -> Result<Vec<User>, sqlx::Error> {
       self.service.find_all(page, size, order).await
    }
}
