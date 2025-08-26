use serde::{Deserialize, Serialize};

/// Main configuration structure for the ODIS Signer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerConfig {
    /// Server configuration
    pub server: ServerConfig,

    /// Database configuration
    pub database: DatabaseConfig,

    /// Keystore configuration
    pub keystore: KeystoreConfig,

    /// Blockchain configuration
    pub blockchain: BlockchainConfig,

    /// API configuration
    pub api: ApiConfig,

    /// Logging configuration
    pub logging: LoggingConfig,
}

/// HTTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server port
    #[serde(default = "default_port")]
    pub port: u16,

    /// Server host
    #[serde(default = "default_host")]
    pub host: String,

    /// SSL certificate path (optional)
    pub ssl_cert_path: Option<String>,

    /// SSL key path (optional)  
    pub ssl_key_path: Option<String>,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database type (postgres, mysql, sqlite)
    pub database_type: String,

    /// Database host
    pub host: String,

    /// Database port
    pub port: Option<u16>,

    /// Database name
    pub database: String,

    /// Database user
    pub user: String,

    /// Database password
    pub password: String,

    /// Use SSL
    #[serde(default)]
    pub ssl: bool,

    /// Connection pool size
    #[serde(default = "default_pool_size")]
    pub pool_max_size: u32,

    /// Connection timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

/// Keystore configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystoreConfig {
    /// Keystore provider (azure, google, aws, mock)
    pub provider: String,

    /// Azure Key Vault configuration
    pub azure: Option<AzureConfig>,

    /// Google Secret Manager configuration
    pub google: Option<GoogleConfig>,

    /// AWS Secrets Manager configuration
    pub aws: Option<AwsConfig>,
}

/// Azure Key Vault configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureConfig {
    /// Key Vault URL
    pub vault_url: String,

    /// Client ID
    pub client_id: Option<String>,

    /// Client secret
    pub client_secret: Option<String>,

    /// Tenant ID
    pub tenant_id: Option<String>,
}

/// Google Secret Manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleConfig {
    /// Google Cloud project ID
    pub project_id: String,

    /// Service account key path
    pub service_account_key_path: Option<String>,
}

/// AWS Secrets Manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    /// AWS region
    pub region: String,

    /// Access key ID
    pub access_key_id: Option<String>,

    /// Secret access key
    pub secret_access_key: Option<String>,
}

/// Blockchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// Blockchain provider URL
    pub provider: String,

    /// API key for blockchain provider
    pub api_key: Option<String>,
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Enable PNP API
    #[serde(default = "default_true")]
    pub pnp_enabled: bool,

    /// Enable Domain API
    #[serde(default = "default_true")]
    pub domain_enabled: bool,

    /// API timeout in seconds
    #[serde(default = "default_api_timeout")]
    pub timeout: u64,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Log format (json, pretty)
    #[serde(default = "default_log_format")]
    pub format: String,
}

// Default values
fn default_port() -> u16 {
    3000
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_pool_size() -> u32 {
    10
}

fn default_timeout() -> u64 {
    30
}

fn default_api_timeout() -> u64 {
    10
}

fn default_true() -> bool {
    true
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "json".to_string()
}

impl Default for SignerConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                port: default_port(),
                host: default_host(),
                ssl_cert_path: None,
                ssl_key_path: None,
            },
            database: DatabaseConfig {
                database_type: "sqlite".to_string(),
                host: "localhost".to_string(),
                port: None,
                database: "odis_signer.db".to_string(),
                user: "signer".to_string(),
                password: "password".to_string(),
                ssl: false,
                pool_max_size: default_pool_size(),
                timeout: default_timeout(),
            },
            keystore: KeystoreConfig {
                provider: "mock".to_string(),
                azure: None,
                google: None,
                aws: None,
            },
            blockchain: BlockchainConfig {
                provider: "http://localhost:8545".to_string(),
                api_key: None,
            },
            api: ApiConfig {
                pnp_enabled: default_true(),
                domain_enabled: default_true(),
                timeout: default_api_timeout(),
            },
            logging: LoggingConfig {
                level: default_log_level(),
                format: default_log_format(),
            },
        }
    }
}
