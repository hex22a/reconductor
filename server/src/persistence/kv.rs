use std::time::Duration;

use fred::{
    prelude::{ClientLike, Config, Pool, TcpConfig},
    types::Builder,
};

pub struct KvProvider {
    client: Pool,
}

impl KvProvider {
    pub async fn new(url: &str, pool_size: usize) -> Result<Self, fred::error::Error> {
        let config = Config::from_url(url)?;
        let client = Builder::from_config(config)
            .with_connection_config(|config| {
                config.connection_timeout = Duration::from_secs(5);
                config.tcp = TcpConfig {
                    nodelay: Some(true),
                    ..Default::default()
                };
            })
            .build_pool(pool_size)?;
        client.init().await?;
        Ok(Self { client })
    }
}
