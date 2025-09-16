pub(crate) use tower_sessions_redis_store::fred::prelude::{
    Config as FredConfig, Pool as FredPool, *,
};
use tracing::info;

use crate::app::App;

impl App {
    pub(super) async fn setup_redis_fred() -> color_eyre::Result<FredPool> {
        info!("Redis Fred: Connecting to Redis (to manage sessions)...");

        let db_num = 0u8;

        let redis_url = std::env::var("REDIS_URL")?;
        let redis_url = format!("{}/{}", redis_url, db_num);

        let config = FredConfig::from_url(&redis_url)?;

        let pool = FredPool::new(config, None, None, Some(ReconnectPolicy::default()), 6)?;

        pool.init().await?;

        info!("Redis Fred: Connected to Redis (to manage sessions)");

        Ok(pool)
    }
}
