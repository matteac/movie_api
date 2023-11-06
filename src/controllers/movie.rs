use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};

use crate::models::postgres::MovieModel;
use crate::schemas::movie::{EditMovie, NewMovie};
use crate::{Database, SharedState};

pub struct MovieController;

impl MovieController {
    pub async fn get_all(
        State(state): State<SharedState>,
    ) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
        match &state.pool {
            Database::Postgres(pool) => match MovieModel::get_all(pool).await {
                Ok(movies) => Ok(Json(movies)),
                Err(_) => Err((StatusCode::NOT_FOUND, "Not Found")),
            },
            Database::Sqlite(_) => {
                todo!();
            }
        }
    }
    pub async fn get_by_id(
        State(state): State<SharedState>,
        Path(id): Path<uuid::Uuid>,
    ) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
        match &state.pool {
            Database::Postgres(pool) => match MovieModel::get_by_id(id, pool).await {
                Ok(movie) => Ok(Json(movie)),
                Err(_) => Err((StatusCode::NOT_FOUND, "Not Found")),
            },
            Database::Sqlite(_) => {
                todo!();
            }
        }
    }

    pub async fn get_by_title(
        State(state): State<SharedState>,
        Path(title): Path<String>,
    ) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
        match &state.pool {
            Database::Postgres(pool) => match MovieModel::get_by_title(&title, pool).await {
                Ok(movies) => Ok(Json(movies)),
                Err(_) => Err((StatusCode::NOT_FOUND, "Not Found")),
            },
            Database::Sqlite(_) => {
                todo!();
            }
        }
    }

    pub async fn get_by_genre(
        State(state): State<SharedState>,
        Path(genre): Path<String>,
    ) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
        match &state.pool {
            Database::Postgres(pool) => match MovieModel::get_by_genre(&genre, pool).await {
                Ok(movies) => Ok(Json(movies)),
                Err(_) => Err((StatusCode::NOT_FOUND, "Not Found")),
            },
            Database::Sqlite(_) => {
                todo!();
            }
        }
    }

    pub async fn get_by_director(
        State(state): State<SharedState>,
        Path(director): Path<String>,
    ) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
        match &state.pool {
            Database::Postgres(pool) => match MovieModel::get_by_director(&director, pool).await {
                Ok(movies) => Ok(Json(movies)),
                Err(_) => Err((StatusCode::NOT_FOUND, "Not Found")),
            },
            Database::Sqlite(_) => {
                todo!();
            }
        }
    }

    pub async fn get_by_year(
        State(state): State<SharedState>,
        Path(year): Path<i32>,
    ) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
        match &state.pool {
            Database::Postgres(pool) => match MovieModel::get_by_year(year, pool).await {
                Ok(movies) => Ok(Json(movies)),
                Err(_) => Err((StatusCode::NOT_FOUND, "Not Found")),
            },
            Database::Sqlite(_) => {
                todo!();
            }
        }
    }

    pub async fn create(
        State(state): State<SharedState>,
        Json(new_movie): Json<NewMovie>,
    ) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
        match &state.pool {
            Database::Postgres(pool) => match MovieModel::create(&new_movie, pool).await {
                Ok(movie) => Ok(Json(movie)),
                Err(_) => Err((StatusCode::NOT_FOUND, "Movie not created")),
            },
            Database::Sqlite(_) => {
                todo!();
            }
        }
    }

    pub async fn update(
        State(state): State<SharedState>,
        Path(id): Path<uuid::Uuid>,
        Json(edit_movie): Json<EditMovie>,
    ) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
        match &state.pool {
            Database::Postgres(pool) => match MovieModel::update(id, edit_movie, pool).await {
                Ok(movie) => Ok(Json(movie)),
                Err(_) => Err((StatusCode::NOT_FOUND, "Movie not updated")),
            },
            Database::Sqlite(_) => {
                todo!();
            }
        }
    }

    pub async fn delete(
        State(state): State<SharedState>,
        Path(id): Path<uuid::Uuid>,
    ) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
        match &state.pool {
            Database::Postgres(pool) => match MovieModel::delete(id, pool).await {
                Ok(_) => Ok("Movie deleted"),
                Err(_) => Err((StatusCode::NOT_FOUND, "Movie not deleted")),
            },
            Database::Sqlite(_) => {
                todo!();
            }
        }
    }
}
