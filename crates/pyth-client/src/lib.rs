mod types;

use reqwest::Client;
use reqwest::IntoUrl;
use serde_json::Value;
use types::AssetType;
use url::Url;

pub const HERMES_BASE: &str = "https://hermes.pyth.network";
pub const BENCHMARKS_BASE: &str = "https://benchmarks.pyth.network";

pub struct PythClient {
    http: Client,
    hermes_base: Url,
    benchmarks_base: Url,
}

impl Default for PythClient {
    fn default() -> Self {
        Self {
            http: Client::new(),
            hermes_base: Url::parse(HERMES_BASE).expect("invalid HERMES_BASE"),
            benchmarks_base: Url::parse(BENCHMARKS_BASE).expect("invalid BENCHMARKS_BASE"),
        }
    }
}

impl PythClient {
    pub fn with_base<H: IntoUrl, B: IntoUrl>(hermes_base: H, benchmarks_base: B) -> Self {
        Self {
            http: Client::new(),
            hermes_base: hermes_base.into_url().expect("invalid hermes_base"),
            benchmarks_base: benchmarks_base.into_url().expect("invalid benchmarks_base"),
        }
    }

    /// GET /v2/price_feeds from Hermes.
    /// Optional filters:
    /// - `query`: case-insensitive substring match on symbol (e.g., "bitcoin")
    /// - `asset_type`: one of:
    ///     "crypto", "fx", "equity", "metal", "rates",
    ///     "crypto_redemption_rate", "commodities", "crypto_index", "crypto_nav"
    pub async fn price_feeds(
        &self,
        query: Option<&str>,
        asset_type: Option<&AssetType>,
    ) -> Result<Value> {
        // Build: <base>/v2/price_feeds?query=...&asset_type=...
        let mut url: Url = self.hermes_base.join("/v2/price_feeds")?;
    }
}

