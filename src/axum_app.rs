use axum::response::Html;
use axum::routing::IntoMakeService;
use axum::{routing::get, Router};

async fn home() -> Html<&'static str> {
    Html("<h1>Home page</h1>")
}

pub fn http_app() -> IntoMakeService<Router> {
    Router::new()
        .route("/", get(home))
        .into_make_service()
}
