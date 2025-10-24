use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, AsRefStr, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AssetType {
    Crypto,
    Fx,
    Equity,
    Metal,
    Rates,
    CryptoRedemptionRate,
    Commodities,
    CryptoIndex,
    CryptoNav,
}
