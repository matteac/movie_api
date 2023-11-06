use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing;
use axum::Router;

use crate::controllers::MovieController;
use crate::AppState;

#[allow(dead_code)]
async fn dummy_handler(
    State(_): State<Arc<AppState>>,
) -> Result<(StatusCode, impl IntoResponse), (StatusCode, impl IntoResponse)> {
    Ok::<(StatusCode, Html<&'static str>), (StatusCode, Html<&'static str>)>((
        StatusCode::OK,
        Html("<h1>dummy</h1>"),
    ))
}

pub fn build_router() -> axum::Router<Arc<AppState>> {
    Router::new()
        .route("/movies", routing::get(MovieController::get_all))
        .route("/movies/:id", routing::get(MovieController::get_by_id)) // get by id
        .route(
            "/movies/title/:title",
            routing::get(MovieController::get_by_title),
        )
        .route(
            "/movies/genre/:genre",
            routing::get(MovieController::get_by_genre),
        )
        .route(
            "/movies/director/:director",
            routing::get(MovieController::get_by_director),
        )
        .route(
            "/movies/year/:year",
            routing::get(MovieController::get_by_year),
        )
        .route("/movies", routing::post(MovieController::create))
        .route("/movies/:id", routing::patch(MovieController::update))
        .route("/movies/:id", routing::delete(MovieController::delete))
}
