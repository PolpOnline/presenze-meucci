mod app;
mod middleware;
mod users;
mod web;

use color_eyre::Result;
use dotenvy::dotenv;
use rustls::crypto::aws_lc_rs;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| {
                "axum_login=info,tower_sessions=info,sqlx=warn,tower_http=debug,\
                 presenze_meucci=debug"
                    .into()
            },
        )))
        .try_init()?;

    dotenv().unwrap_or_default();

    aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install AWS LC provider");

    let app = App::new().await?;

    // Later we might want to add CLI commands here, e.g. for seeding the database.
    app.serve().await?;

    Ok(())
}
