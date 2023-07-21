use axum::response::Html;

#[tracing::instrument]
pub async fn health_check() -> Html<&'static str> {
    tracing::event!(target: "backend", tracing::Level::DEBUG, "Accessing health-check endpoint.");
    Html("Application is up and running :)")
}
