use crate::schemas::movie::{EditMovie, Movie, NewMovie, ResponseMovie};

pub struct MovieModel;

#[allow(dead_code)]
impl MovieModel {
    pub async fn get_all(db: &sqlx::postgres::PgPool) -> Result<Vec<ResponseMovie>, sqlx::Error> {
        let movies = sqlx::query_as!(
            ResponseMovie,
            r#"SELECT m.*, array_agg(g.name) AS genres
FROM movie m
LEFT JOIN movie_genres mg ON m.id = mg.movie_id
LEFT JOIN genre g ON mg.genre_id = g.id
GROUP BY m.id;"#
        )
        .fetch_all(db)
        .await?;
        Ok(movies)
    }

    pub async fn get_by_id(
        id: uuid::Uuid,
        db: &sqlx::postgres::PgPool,
    ) -> Result<ResponseMovie, sqlx::Error> {
        let movie = sqlx::query_as!(
            ResponseMovie,
            r#"SELECT m.*, array_agg(g.name) AS genres
FROM movie m
LEFT JOIN movie_genres mg ON m.id = mg.movie_id
LEFT JOIN genre g ON mg.genre_id = g.id
WHERE m.id = $1
GROUP BY m.id;
"#,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(movie)
    }

    pub async fn get_by_title(
        title: &String,
        db: &sqlx::postgres::PgPool,
    ) -> Result<Vec<ResponseMovie>, sqlx::Error> {
        let movies = sqlx::query_as!(
            ResponseMovie,
            r#"SELECT m.*, array_agg(g.name) AS genres
FROM movie m
LEFT JOIN movie_genres mg ON m.id = mg.movie_id
LEFT JOIN genre g ON mg.genre_id = g.id
WHERE LOWER(m.title) LIKE $1
GROUP BY m.id;
"#,
            title.to_lowercase(),
        )
        .fetch_all(db)
        .await?;
        Ok(movies)
    }

    pub async fn get_by_genre(
        genre: &String,
        db: &sqlx::postgres::PgPool,
    ) -> Result<Vec<ResponseMovie>, sqlx::Error> {
        let movies = sqlx::query_as!(
            ResponseMovie,
            r#"SELECT m.*, (
    SELECT array_agg(g.name)
    FROM movie_genres mg
    JOIN genre g ON mg.genre_id = g.id
    WHERE mg.movie_id = m.id
) AS genres
FROM movie m
WHERE EXISTS (
    SELECT 1
    FROM movie_genres mg
    JOIN genre g ON mg.genre_id = g.id
    WHERE mg.movie_id = m.id
    AND LOWER(g.name) LIKE $1
);
"#,
            genre.to_lowercase()
        )
        .fetch_all(db)
        .await?;
        Ok(movies)
    }

    pub async fn get_by_director(
        director: &String,
        db: &sqlx::postgres::PgPool,
    ) -> Result<Vec<ResponseMovie>, sqlx::Error> {
        let movies = sqlx::query_as!(
            ResponseMovie,
            r#"
SELECT m.*, array_agg(g.name) AS genres
FROM movie m
LEFT JOIN movie_genres mg ON m.id = mg.movie_id
LEFT JOIN genre g ON mg.genre_id = g.id
WHERE LOWER(m.director) LIKE $1
GROUP BY m.id;
        "#,
            director.to_lowercase()
        )
        .fetch_all(db)
        .await?;
        Ok(movies)
    }

    pub async fn get_by_year(
        year: i32,
        db: &sqlx::postgres::PgPool,
    ) -> Result<Vec<ResponseMovie>, sqlx::Error> {
        let movies = sqlx::query_as!(
            ResponseMovie,
            r#"
SELECT m.*, array_agg(g.name) AS genres
FROM movie m
LEFT JOIN movie_genres mg ON m.id = mg.movie_id
LEFT JOIN genre g ON mg.genre_id = g.id
WHERE m.year = $1
GROUP BY m.id;
        "#,
            year
        )
        .fetch_all(db)
        .await?;
        Ok(movies)
    }

    pub async fn create(
        new_movie: &NewMovie,
        db: &sqlx::postgres::PgPool,
    ) -> Result<Movie, sqlx::Error> {
        let movie = sqlx::query_as!(
            Movie,
            r#"
            INSERT INTO movie (id, title, year, director, poster, duration_in_seconds, rate)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, title, year, director, poster, duration_in_seconds, rate as "rate: f32"
            "#,
            uuid::Uuid::new_v4(),
            new_movie.title,
            new_movie.year,
            new_movie.director,
            new_movie.poster,
            new_movie.duration_in_seconds,
            new_movie.rate
        )
        .fetch_one(db)
        .await?;
        for genre in &new_movie.genres {
            sqlx::query!(
                "INSERT INTO movie_genres (movie_id, genre_id) VALUES ($1, (SELECT id FROM genre WHERE name = $2))",
                movie.id,
                genre
            )
            .execute(db)
            .await?;
        }
        Ok(movie)
    }

    pub async fn update(
        id: uuid::Uuid,
        edit_movie: EditMovie,
        db: &sqlx::postgres::PgPool,
    ) -> Result<Movie, sqlx::Error> {
        let actual_movie = MovieModel::get_by_id(id, db).await?;
        let movie = Movie {
            id,
            title: edit_movie.title.unwrap_or(actual_movie.title),
            year: edit_movie.year.unwrap_or(actual_movie.year),
            director: edit_movie.director.unwrap_or(actual_movie.director),
            poster: edit_movie.poster.unwrap_or(actual_movie.poster),
            duration_in_seconds: edit_movie
                .duration_in_seconds
                .unwrap_or(actual_movie.duration_in_seconds),
            rate: edit_movie.rate.unwrap_or(actual_movie.rate),
        };
        let movie = sqlx::query_as!(
            Movie,
            r#"
            UPDATE movie
            SET title = $1, year = $2, director = $3, poster = $4, duration_in_seconds = $5, rate = $6
            WHERE id = $7
            RETURNING id, title, year, director, poster, duration_in_seconds, rate as "rate: f32"
            "#,
            movie.title,
            movie.year,
            movie.director,
            movie.poster,
            movie.duration_in_seconds,
            movie.rate,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(movie)
    }

    pub async fn delete(id: uuid::Uuid, db: &sqlx::postgres::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM movie_genres WHERE movie_id = $1", id)
            .execute(db)
            .await?;
        sqlx::query!("DELETE FROM movie WHERE id = $1", id)
            .execute(db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::postgres::MovieModel;
    use crate::schemas::movie::{EditMovie, NewMovie, ResponseMovie};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[tokio::test]
    async fn crud() {
        let mut pool = sqlx::postgres::PgPoolOptions::new()
            .connect("postgres://postgres:matteac1331@localhost:5432/postgres")
            .await
            .unwrap();
        let pre_ops = MovieModel::get_all(&mut pool).await.unwrap();

        // create a movie
        let new_movie = NewMovie {
            title: "test_movie_1".to_string(),
            year: 2022,
            director: "test_director_1".to_string(),
            genres: vec!["Drama".to_string()],
            poster: "test_poster_1".to_string(),
            duration_in_seconds: 120,
            rate: 4.5,
        };

        let movie = MovieModel::create(&new_movie, &mut pool).await.unwrap();
        let expected_movie = ResponseMovie {
            id: movie.id,
            title: movie.title,
            year: movie.year,
            director: movie.director,
            genres: Some(new_movie.genres),
            poster: movie.poster,
            duration_in_seconds: movie.duration_in_seconds,
            rate: movie.rate,
        };
        assert_eq!(
            expected_movie,
            MovieModel::get_by_id(movie.id, &mut pool).await.unwrap()
        );

        // update the movie
        let edited_movie = EditMovie {
            title: Some("test_movie_2".to_string()),
            year: Some(2023),
            director: Some("test_director_2".to_string()),
            ..Default::default()
        };
        let movie = MovieModel::update(movie.id, edited_movie.clone(), &mut pool)
            .await
            .unwrap();
        // assert edited fields
        assert_eq!(movie.title, edited_movie.title.unwrap());
        assert_eq!(movie.year, edited_movie.year.unwrap());
        assert_eq!(movie.director, edited_movie.director.unwrap());

        // delete the movie
        MovieModel::delete(movie.id, &mut pool).await.unwrap();

        // movies after the create, update and delete if the test movie
        let post_ops = MovieModel::get_all(&mut pool).await.unwrap();
        assert_eq!(pre_ops, post_ops);
    }
}
