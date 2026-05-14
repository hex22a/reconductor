use std::{collections::HashMap, time::Duration};

use fred::{
    prelude::{ClientLike, Config, HashesInterface, KeysInterface, Pool, TcpConfig},
    types::Builder,
};

pub trait KvProvider {
    fn get(
        &self,
        key: &str,
    ) -> impl Future<Output = Result<Option<String>, fred::error::Error>> + Send;
    fn set(
        &self,
        key: &str,
        value: &str,
        ttl: Option<&Duration>,
    ) -> impl Future<Output = Result<(), fred::error::Error>> + Send;
    fn exists(&self, key: &str) -> impl Future<Output = Result<bool, fred::error::Error>> + Send;
    fn hgetall(
        &self,
        key: &str,
    ) -> impl Future<Output = Result<HashMap<String, String>, fred::error::Error>> + Send;
    fn hset(
        &self,
        key: &str,
        values: HashMap<String, String>,
    ) -> impl Future<Output = Result<(), fred::error::Error>> + Send;
    fn expire(
        &self,
        key: &str,
        ttl: &Duration,
    ) -> impl Future<Output = Result<(), fred::error::Error>> + Send;
    fn del(&self, key: &str) -> impl Future<Output = Result<(), fred::error::Error>> + Send;
}

#[derive(Clone)]
pub struct FredKvProvider {
    client: Pool,
}

pub async fn init_kv(url: &str, pool_size: usize) -> FredKvProvider {
    let config = Config::from_url(url).expect("unable to get config from URL");
    let client = Builder::from_config(config)
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(5);
            config.tcp = TcpConfig {
                nodelay: Some(true),
                ..Default::default()
            };
        })
        .build_pool(pool_size)
        .expect("unable to build KV clinet");
    client.init().await.expect("unable to init KV store");
    FredKvProvider { client }
}

impl KvProvider for FredKvProvider {
    async fn get(&self, key: &str) -> Result<Option<String>, fred::error::Error> {
        self.client.get(key).await
    }

    async fn set(
        &self,
        key: &str,
        value: &str,
        ttl: Option<&Duration>,
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

    async fn expire(&self, key: &str, ttl: &Duration) -> Result<(), fred::error::Error> {
        self.client.expire(key, ttl.as_secs() as i64, None).await
    }

    async fn del(&self, key: &str) -> Result<(), fred::error::Error> {
        self.client.del(key).await
    }
}
