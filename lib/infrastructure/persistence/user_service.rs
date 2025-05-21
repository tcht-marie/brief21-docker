use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::domain::entities::User;

#[derive(Clone)]
pub struct UserService {
    db: Arc<Pool<Postgres>>,
}

// const FIELD_LIST: [&str; 4] = ["\"id\"", "\"username\"", "\"email\"", "\"password\""];
const WITHOUT_ID: [&str; 3] = ["\"username\"", "\"email\"", "\"password\""];
const WITHOUT_PASSWORD: [&str; 2] = ["\"id\"", "\"username\""];
// const WITHOUT_ID_WITH_NULL_AS_PASSWORD: [&str; 3] = ["\"username\"", "\"email\"", "null as \"password\""];
const WITH_ID_AND_NULL_AS_PASSWORD: [&str; 4] = ["\"id\"", "\"username\"", "\"email\"", "null as \"password\""];

impl UserService {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        UserService { db }
    }

    pub async fn save(self, user: User) -> Result<User, sqlx::Error> {
        sqlx
            ::query_as(
                &format!("INSERT INTO \"users\" ({}) VALUES ($1, $2) RETURNING {}", WITHOUT_PASSWORD.join(","), WITH_ID_AND_NULL_AS_PASSWORD.join(","))
            )
            .bind(user.username())
            .bind(user.email())
            .fetch_one(&*self.db).await
    }

    pub async fn update(self, user: User) -> Result<User, sqlx::Error> {
        sqlx
            ::query_as(
                &format!("UPDATE \"users\" SET \"username\" = $1, \"email\" = $2 WHERE \"id\" = $3 RETURNING {}", WITH_ID_AND_NULL_AS_PASSWORD.join(","))
            )
            .bind(user.username())
            .bind(user.email())
            .bind(user.id())
            .fetch_one(&*self.db).await
    }
    
    pub async fn save_password(self, user: User) -> Result<User, sqlx::Error> {
        sqlx
            ::query_as(
                &format!("INSERT INTO \"users\" ({}) VALUES ($1, $2, $3) RETURNING {}", WITHOUT_ID.join(","), WITH_ID_AND_NULL_AS_PASSWORD.join(","))
            )
            .bind(user.username())
            .bind(user.email())
            .bind(user.password())
            .fetch_one(&*self.db).await
    }
    
    pub async fn update_password(self, user: User) -> Result<User, sqlx::Error> {
        sqlx
            ::query_as(
                &format!("UPDATE \"users\" SET \"username\" = $1, \"email\" = $2, \"password\" = $3 WHERE \"id\" = $4 RETURNING  {}", WITH_ID_AND_NULL_AS_PASSWORD.join(","))
            )
            .bind(user.username())
            .bind(user.email())
            .bind(user.password())
            .bind(user.id())
            .fetch_one(&*self.db).await
    }

    pub async fn find_by_id(self, user_id: i32) -> Result<User, sqlx::Error> {
        sqlx
            ::query_as(&format!("SELECT {} FROM \"users\" WHERE \"id\" = $1", WITH_ID_AND_NULL_AS_PASSWORD.join(",")))
            .bind(user_id)
            .fetch_one(&*self.db).await
    }

    pub async fn find_all(self, page: i32, size: i32, order: String) -> Result<Vec<User>, sqlx::Error> {
       sqlx
            ::query_as(&format!("SELECT {} FROM \"users\" ORDER BY $1 LIMIT $2 OFFSET $3", WITH_ID_AND_NULL_AS_PASSWORD.join(",")))
            .bind(order)
            .bind(size)
            .bind(page * size)
            .fetch_all(&*self.db).await
    }

    pub async fn find_by_username_or_email_and_password(self, username: String, password: String) -> Result<User, sqlx::Error> {
        sqlx
            ::query_as(&format!("SELECT {} FROM \"users\" WHERE (\"username\" = $1 OR \"email\" = $1) AND \"password\" = $2", WITH_ID_AND_NULL_AS_PASSWORD.join(",")))
            .bind(username)
            .bind(password)
            .fetch_one(&*self.db).await
    }
}
