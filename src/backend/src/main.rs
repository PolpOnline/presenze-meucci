mod app;
mod fixtures;
mod middleware;
pub mod types;
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
        .with(tracing_subscriber::fmt::layer())
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

    #[cfg(debug_assertions)]
    {
        use app::cli::Args;
        use clap::Parser;

        let args = Args::parse();

        match args.command {
            None => app.serve().await,
            // For future use with fixtures to import static data
            _ => Ok(()),
        }
    }

    // Run the app in production without the CLI
    #[cfg(not(debug_assertions))]
    {
        app.serve().await
    }
}
