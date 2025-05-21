use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::extract::{ Path, State };
use axum_controller::{ controller, route, TypedRouter };
use sqlx::Error::Database;

use crate::application::dto::UserCreateDto;
use crate::AppState;

#[derive(Default)]
pub struct UserController;

#[controller(path = "/api/users", state = AppState)]
impl UserController {
    #[route(GET "/?page&size&order")]
    async fn get_all(
        size: Option<i32>,
        page: Option<i32>,
        order: Option<String>,
        State(state): State<AppState>
    ) -> impl IntoResponse {
        match
            state.user_repository
                .find_all(page.unwrap_or(1), size.unwrap_or(10), order.unwrap_or("id".into())).await
                .inspect(|f| tracing::debug!("{:?}", f))
        {
            Ok(users) => Ok(Json(users)),
            _ => Err(StatusCode::NOT_FOUND),
        }
    }

    #[route(POST "/")]
    async fn create_user(
        State(state): State<AppState>,
        Json(user_request): Json<UserCreateDto>
    ) -> impl IntoResponse {
        tracing::debug!("{:?}", user_request);
        let user = user_request.into();
        match state.user_repository.save_password(user).await {
            Ok(new_user) => Ok(Json(new_user)),
            Err(Database(err)) => {
                match err.code().unwrap().to_string().as_str() {
                    "23505" => {
                        tracing::debug!("{:?}", err);
                        Err((StatusCode::CONFLICT, "User already exists"))
                    }
                    _ => {
                        tracing::error!("Database error: {}", err.message());
                        tracing::debug!("{:?}", err);
                        Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error"))
                    }
                }
            },
            Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "Tried to update a non-existent user")),
            Err(err) => {
                tracing::error!("Internal error: {}", err.to_string());
                tracing::debug!("{:#?}", err);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error"))
            }
        }
    }

    #[route(GET "/{id}")]
    async fn get_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
        match state.user_repository.find_by_id(id).await {
            Ok(user) => Ok(Json(user)),
            _ => Err(StatusCode::NOT_FOUND),
        }
    }

    pub fn get_router(state: AppState) -> axum::Router<AppState> {
        Self::into_router(state)
    }
}
