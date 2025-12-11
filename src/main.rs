mod routes;
mod handlers;
mod state;
mod response;

use axum::Router;
use state::AppState;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL non défini");

    let state = AppState::new(&database_url);

    let app = routes::create_routes().with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Serveur lancé sur http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
