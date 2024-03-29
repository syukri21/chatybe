use axum::{routing::get, Router};

pub mod connection;
pub mod kcloak;
pub mod model;
pub mod routes;
pub mod schema;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let app = Router::new()
        .route("/signup", get(routes::signup::get))
        .route("/signin", get(routes::signin::get));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    kcloak::KCloak::new().await;

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
