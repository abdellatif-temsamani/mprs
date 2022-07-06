#[derive(Debug)]
pub struct ConfigManager {
    pub host: Option<String>,
    pub port: Option<String>,
    pub silent: Option<bool>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            host: Some("127.0.0.1".to_owned()),
            port: Some("6600".to_owned()),
            silent: Some(false),
        }
    }
}
