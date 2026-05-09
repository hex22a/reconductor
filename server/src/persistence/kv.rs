use std::{collections::HashMap, time::Duration};

use fred::{
    prelude::{ClientLike, Config, HashesInterface, KeysInterface, Pool, TcpConfig},
    types::Builder,
};

pub mod csrf;
pub mod session;

pub trait KvProvider {
    fn get(
        &self,
        key: String,
    ) -> impl Future<Output = Result<Option<String>, fred::error::Error>> + Send;
    fn set(
        &self,
        key: String,
        value: String,
        ttl: Option<Duration>,
    ) -> impl Future<Output = Result<(), fred::error::Error>> + Send;
    fn exists(&self, key: String) -> impl Future<Output = Result<bool, fred::error::Error>> + Send;
    fn hgetall(
        &self,
        key: String,
    ) -> impl Future<Output = Result<HashMap<String, String>, fred::error::Error>> + Send;
    fn hset(
        &self,
        key: String,
        values: HashMap<String, String>,
    ) -> impl Future<Output = Result<(), fred::error::Error>> + Send;
    fn expire(
        &self,
        key: String,
        ttl: Duration,
    ) -> impl Future<Output = Result<(), fred::error::Error>> + Send;
    fn del(&self, key: String) -> impl Future<Output = Result<(), fred::error::Error>> + Send;
}

#[derive(Clone)]
pub struct FredKvProvider {
    client: Pool,
}

impl FredKvProvider {
    pub async fn new(url: String, pool_size: usize) -> Result<Self, fred::error::Error> {
        let config = Config::from_url(&url)?;
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
    async fn get(&self, key: String) -> Result<Option<String>, fred::error::Error> {
        self.client.get(key).await
    }

    async fn set(
        &self,
        key: String,
        value: String,
        ttl: Option<Duration>,
    ) -> Result<(), fred::error::Error> {
        let expire = ttl.map(|d| fred::types::Expiration::EX(d.as_secs() as i64));
        self.client.set(key, value, expire, None, false).await
    }

    async fn exists(&self, key: String) -> Result<bool, fred::error::Error> {
        let count: u32 = self.client.exists(key).await?;
        Ok(count > 0)
    }

    async fn hgetall(&self, key: String) -> Result<HashMap<String, String>, fred::error::Error> {
        self.client.hgetall(key).await
    }

    async fn hset(
        &self,
        key: String,
        values: HashMap<String, String>,
    ) -> Result<(), fred::error::Error> {
        self.client.hset(key, values).await
    }

    async fn expire(&self, key: String, ttl: Duration) -> Result<(), fred::error::Error> {
        self.client.expire(key, ttl.as_secs() as i64, None).await
    }

    async fn del(&self, key: String) -> Result<(), fred::error::Error> {
        self.client.del(key).await
    }
}
