use configr::{Config, Configr};
use serde::{Deserialize, Serialize};

#[derive(Configr, Deserialize, Serialize, Default, Clone )]
pub struct MpdConfig {
    pub host: String,
    pub port: String,
}
