mod handler;
mod model;
mod response;
mod route;
use route::router;

#[tokio::main]
async fn main() {
    let app = router();

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
