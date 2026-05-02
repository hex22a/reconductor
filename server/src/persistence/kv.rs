use std::{collections::HashMap, time::Duration};

use fred::{
    prelude::{ClientLike, Config, HashesInterface, KeysInterface, Pool, TcpConfig},
    types::Builder,
};

pub mod session;

#[allow(async_fn_in_trait)]
pub trait KvProvider {
    async fn get(&self, key: &str) -> Result<Option<String>, fred::error::Error>;
    async fn set(
        &self,
        key: &str,
        value: &str,
        ttl: Option<Duration>,
    ) -> Result<(), fred::error::Error>;
    async fn exists(&self, key: &str) -> Result<bool, fred::error::Error>;
    async fn hgetall(&self, key: &str) -> Result<HashMap<String, String>, fred::error::Error>;
    async fn hset(
        &self,
        key: &str,
        values: HashMap<String, String>,
    ) -> Result<(), fred::error::Error>;
    async fn expire(&self, key: &str, ttl: Duration) -> Result<(), fred::error::Error>;
    async fn del(&self, key: &str) -> Result<(), fred::error::Error>;
}

pub struct FredKvProvider {
    client: Pool,
}

impl FredKvProvider {
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

impl KvProvider for FredKvProvider {
    async fn get(&self, key: &str) -> Result<Option<String>, fred::error::Error> {
        self.client.get(key).await
    }

    async fn set(
        &self,
        key: &str,
        value: &str,
        ttl: Option<Duration>,
    ) -> Result<(), fred::error::Error> {
        let expire = ttl.map(|d| fred::types::Expiration::EX(d.as_secs() as i64));
        self.client.set(key, value, expire, None, false).await
    }

    async fn exists(&self, key: &str) -> Result<bool, fred::error::Error> {
        let count: u32 = self.client.exists(key).await?;
        Ok(count > 0)
    }

    async fn hgetall(&self, key: &str) -> Result<HashMap<String, String>, fred::error::Error> {
        self.client.hgetall(key).await
    }

    async fn hset(
        &self,
        key: &str,
        values: HashMap<String, String>,
    ) -> Result<(), fred::error::Error> {
        self.client.hset(key, values).await
    }

    async fn expire(&self, key: &str, ttl: Duration) -> Result<(), fred::error::Error> {
        self.client.expire(key, ttl.as_secs() as i64, None).await
    }

    async fn del(&self, key: &str) -> Result<(), fred::error::Error> {
        self.client.del(key).await
    }
}
