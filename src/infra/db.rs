use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub async fn db_init(database_url: String) -> Result<DatabaseConnection, DbErr> {
    // Mask password for logging
    let masked_url = if let Some(at_idx) = database_url.find('@') {
        if let Some(proto_idx) = database_url.find("://") {
            format!(
                "{}:[MASKED]@{}",
                &database_url[..proto_idx + 3],
                &database_url[at_idx + 1..]
            )
        } else {
            "URL (malformed proto)".to_string()
        }
    } else {
        database_url.clone()
    };

    tracing::info!("DATABASE_URL: {}", masked_url);

    tracing::info!("Connecting to database: {}", masked_url);

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(300))
        .sqlx_logging(true);

    let db = Database::connect(opt).await?;

    tracing::info!("Database connection established");

    Ok(db)
}
