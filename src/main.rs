mod routes;
mod handlers;
mod response;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = routes::create_routes();

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Serveur lancé sur http://127.0.0.1:3000/health");

    axum::serve(listener, app)
        .await
        .unwrap();
}
