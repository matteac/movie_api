use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct NewMovie {
    pub title: String,
    pub year: i32,
    pub director: String,
    pub genres: Vec<String>,
    pub poster: String,
    pub duration_in_seconds: i32,
    pub rate: f32,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ResponseMovie {
    pub id: uuid::Uuid,
    pub title: String,
    pub year: i32,
    pub director: String,
    pub genres: Option<Vec<String>>,
    pub poster: String,
    pub duration_in_seconds: i32,
    pub rate: f32,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default)]
pub struct EditMovie {
    pub title: Option<String>,
    pub year: Option<i32>,
    pub director: Option<String>,
    pub genres: Option<Vec<String>>,
    pub poster: Option<String>,
    pub duration_in_seconds: Option<i32>,
    pub rate: Option<f32>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, PartialEq)]
pub struct Movie {
    pub id: uuid::Uuid,
    pub title: String,
    pub year: i32,
    pub director: String,
    pub poster: String,
    pub duration_in_seconds: i32,
    pub rate: f32,
}

#[allow(dead_code)]
pub struct Genre {
    pub id: i32,
    pub name: String,
}

#[allow(dead_code)]
pub struct MovieGenres {
    pub id: String, // movie_id + genre_id
    pub movie_id: sqlx::types::uuid::Uuid,
    pub genre_id: i32,
}
