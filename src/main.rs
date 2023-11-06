mod controllers;
mod models;
mod routes;
mod schemas;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Database {
    Postgres(sqlx::postgres::PgPool),
    Sqlite(sqlx::sqlite::SqlitePool),
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct AppState {
    pub pool: Database,
}

type SharedState = std::sync::Arc<AppState>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "movie_api=info,tower_http=info,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env!("DATABASE_URL");

    let pool = Database::Postgres(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?,
    );

    let state = SharedState::new(AppState { pool });

    let api_route = axum::Router::new().merge(routes::movie::build_router().with_state(state));
    let app = axum::Router::new()
        .nest("/api", api_route)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
