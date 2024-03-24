use axum::{routing::get, Router}; // 0.6.1Router;
mod routes;
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/signup", get(routes::signup::get))
        .route("/signin", get(routes::signin::get));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
