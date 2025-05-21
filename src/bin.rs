use std::sync::Arc;

use axum::{ http::StatusCode, Router, response::IntoResponse };
use rust_embed::RustEmbed;
use sqlx::{ migrate::Migrator, postgres::PgPoolOptions };
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{ layer::SubscriberExt, util::SubscriberInitExt };

use shop_lib::{
    controllers::UserController,
    domain::repositories::UserRepository,
    infrastructure::persistence::UserService,
    AppState,
};

static MIGRATOR: Migrator = sqlx::migrate!("db/migrations");

#[derive(RustEmbed)]
#[folder = "public/"]
struct Assets;

#[tokio::main]
async fn main() {
    tracing_subscriber
        ::registry()
        .with(
            tracing_subscriber::EnvFilter
                ::try_from_default_env()
                .unwrap_or_else(|_| "shop_bin=info,shop_lib=info".into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = format!(
        "postgres://{}:{}@{}/{}",
        std::env::var("POSTGRES_USER").unwrap_or("postgres".to_string()),
        std::env::var("POSTGRES_PASSWORD").unwrap_or("postgres".to_string()),
        std::env::var("POSTGRES_HOST").unwrap_or("localhost:5432".to_string()),
        std::env::var("POSTGRES_DB").unwrap_or("app".to_string())
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await
        .expect("Failed to connect to database");
    let arc_pool = Arc::new(pool);

    if let Err(err) = MIGRATOR.run(&*arc_pool).await {
        tracing::error!("Failed to run migrations: {}", err);
        tracing::debug!("{:?}", err);
        return;
    }

    let user_service = UserService::new(arc_pool);
    let user_repository = UserRepository::new(user_service);
    let app_state = AppState::new(user_repository);

    let user_controller: axum::Router<AppState> = UserController::get_router(app_state.clone());
    let app = Router::new().merge(user_controller).with_state(app_state);

    let listen_addr = format!(
        "{}:{}",
        std::env::var("HOST").unwrap_or("127.0.0.1".to_string()),
        std::env::var("PORT").unwrap_or("8080".to_string()).parse::<i32>().expect("Invalid port")
    );
    serve(app, listen_addr.parse::<SocketAddr>().expect("Invalid address")).await;
}

async fn serve_static(uri: axum::http::Uri) -> impl IntoResponse {
    let request_path = uri.path().trim_start_matches('/');
    let asset_path = if request_path.len() == 0 || request_path == "/" {
        "index.html" 
    } else { 
        request_path
    };
    
    match Assets::get(asset_path) {
        Some(content) => {
            let mime = mime_guess::from_path(asset_path).first_or_octet_stream();
            (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}

async fn serve(app: Router, addr: SocketAddr) {
    let app = app.fallback(serve_static);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http())).await.unwrap();
}
