pub mod inr_balance;
pub mod orderbook;
pub mod routes;
pub mod stock_balance;
use routes::create_routes;

pub async fn run() {
    let app = create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is running on port 3000");
    axum::serve(listener, app).await.unwrap();
}
