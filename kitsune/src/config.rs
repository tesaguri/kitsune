use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::{num::NonZeroUsize, path::Path};
use tokio::fs;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RedisCacheConfiguration {
    pub url: SmolStr,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum CacheConfiguration {
    Redis(RedisCacheConfiguration),
    InMemory,
    None,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DatabaseConfiguration {
    pub url: SmolStr,
    pub max_connections: u32,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EmailConfiguration {
    pub from_address: SmolStr,
    pub host: SmolStr,
    pub username: SmolStr,
    pub password: SmolStr,
    pub starttls: bool,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EmbedConfiguration {
    pub service_url: SmolStr,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum FederationFilterConfiguration {
    Allow { domains: Vec<SmolStr> },
    Deny { domains: Vec<SmolStr> },
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InstanceConfiguration {
    pub name: SmolStr,
    pub description: SmolStr,
    pub character_limit: usize,
    pub federation_filter: FederationFilterConfiguration,
    pub registrations_open: bool,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobQueueConfiguration {
    pub redis_url: SmolStr,
    pub num_workers: NonZeroUsize,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OidcConfiguration {
    pub server_url: SmolStr,
    pub client_id: SmolStr,
    pub client_secret: SmolStr,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RedisMessagingConfiguration {
    pub url: SmolStr,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum MessagingConfiguration {
    Redis(RedisMessagingConfiguration),
    InProcess,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct KitsuneSearchConfiguration {
    pub index_server: SmolStr,
    pub search_servers: Vec<SmolStr>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MeiliSearchConfiguration {
    pub instance_url: SmolStr,
    pub api_key: SmolStr,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum SearchConfiguration {
    Kitsune(KitsuneSearchConfiguration),
    Meilisearch(MeiliSearchConfiguration),
    Sql,
    None,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfiguration {
    pub frontend_dir: SmolStr,
    pub max_upload_size: usize,
    pub media_proxy_enabled: bool,
    pub oidc: Option<OidcConfiguration>,
    pub port: u16,
    pub prometheus_port: u16,
    pub request_timeout_secs: u64,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FsStorageConfiguration {
    pub upload_dir: SmolStr,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct S3StorageConfiguration {
    pub bucket_name: SmolStr,
    pub endpoint_url: SmolStr,
    pub region: SmolStr,
    pub force_path_style: bool,
    pub access_key: SmolStr,
    pub secret_access_key: SmolStr,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum StorageConfiguration {
    Fs(FsStorageConfiguration),
    S3(S3StorageConfiguration),
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UrlConfiguration {
    pub scheme: SmolStr,
    pub domain: SmolStr,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Configuration {
    pub cache: CacheConfiguration,
    pub database: DatabaseConfiguration,
    pub email: Option<EmailConfiguration>,
    pub embed: Option<EmbedConfiguration>,
    pub instance: InstanceConfiguration,
    pub job_queue: JobQueueConfiguration,
    pub messaging: MessagingConfiguration,
    pub server: ServerConfiguration,
    pub search: SearchConfiguration,
    pub storage: StorageConfiguration,
    pub url: UrlConfiguration,
}

impl Configuration {
    pub async fn load<P>(path: P) -> eyre::Result<Self>
    where
        P: AsRef<Path>,
    {
        let content = fs::read_to_string(path).await?;
        toml::from_str(&content).map_err(eyre::Report::from)
    }
}
