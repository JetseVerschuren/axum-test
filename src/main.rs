mod routes;
mod settings;

use std::{net::SocketAddr, str::FromStr};

use axum::Server;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let settings = settings::get_settings().expect("Failed to read settings.");

    tracing_subscriber::fmt::init();

    tracing::event!(target: "backend", tracing::Level::INFO, "Listening on http://127.0.0.1:{}/", settings.application.port);

    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );
    let address = SocketAddr::from_str(&address).unwrap();
    tracing::debug!("listening on {}", address);
    Server::bind(&address)
        .serve(routes::create_router().into_make_service())
        .await
        .unwrap();
}
