use axum::{
    Router,
    routing::{delete, get},
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

use crate::http::*;

mod http;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new(format!("{}=debug", env!("CARGO_CRATE_NAME"))))
                .unwrap(),
        )
        .init();

    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db.sqlite?mode=rwc".to_string());

    let opt = ConnectOptions::new(&db_connection_str);
    let db = Database::connect(opt).await?;

    Migrator::up(&db, None).await?;

    let app_state = AppState { db: db };

    let assets = std::env::var_os("BEWERBUNGS_TOOL_ASSETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets"));

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/firmas", get(get_firma_list))
        .route("/firma/add", get(get_firma_form).post(post_firma))
        .route("/firma/{id}", get(get_firma).put(put_firma)) // todo add put function to handle update to firma
        .route("/contacts", get(get_contact_list))
        .route("/contact/add", get(get_contact_form).post(post_contact))
        .route("/contact/{id}", delete(delete_contact))
        .nest_service("/assets/", ServeDir::new(assets))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    println!("Running on: http://localhost:{}", &port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

pub fn main() {
    let result = start();

    if let Err(err) = result {
        println!("Error: {err}");
    }
}
