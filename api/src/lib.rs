use askama::Template;
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sqlx::{Connection, Pool, Sqlite, SqliteConnection, SqlitePool};
use tower_http::services::ServeDir;

use crate::http::{
    get_contact_form, get_contact_list, get_firma_form, get_firma_list, index, post_contact,
    post_firma,
};

mod http;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db.sqlite?mode=rwc".to_string());

    let opt = ConnectOptions::new(&db_connection_str);
    let db = Database::connect(opt).await?;

    Migrator::up(&db, None).await?;

    let app_state = AppState { db: db };

    let assets = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/firmas", get(get_firma_list))
        .route("/firma/add", get(get_firma_form).post(post_firma))
        .route("/contacts", get(get_contact_list))
        .route("/contact/add", get(get_contact_form).post(post_contact))
        .nest_service("/assets/", ServeDir::new(assets))
        .with_state(app_state);

    println!("Running on: http://localhost:3000");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
