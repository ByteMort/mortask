use mortask_backend::run;
use mortask_backend::background_jobs::start_expired_code_cleanup;

use sqlx::{ PgPool, postgres::PgPoolOptions };
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // tracing_subscriber::fmt::init();
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt::layer())
        .init();

    tracing::info!("Starting the DB Connection...");

    let database_url = std::env::var("DATABASE_URL")
    .expect("No DATABASE_URL");
    
    let pool:PgPool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await
        .unwrap_or_else(|err|{
            tracing::error!("Connection Error: {}", err);
            panic!("Connection Stopped.");
        });

    tracing::info!("Running database migrations...");
    if let Err(err) = sqlx::migrate!("./migrations").run(&pool).await {
        tracing::error!("Failed to run migrations: {}", err);
        panic!("Migration error, stopping server.");
    }
    tracing::info!("Migrations applied successfully!");
    
    start_expired_code_cleanup(pool.clone()).await;
    run(&pool).await;

    tracing::info!("Connection Successfull!");
}
