use super::Param;

#[derive(Debug)]
pub struct ConfigManager {
    pub host: String,
    pub port: String,
    pub silent: bool,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: "6600".to_owned(),
            silent: false,
        }
    }
    pub fn update(&mut self, args: Vec<Param>) {
        for arg in args {
            match &arg.flag as &str {
                "--host" => self.host = arg.value,
                "--port" => self.port = arg.value,
                "--silent" => self.silent = arg.value == *"True",

                &_ => continue,
            }
        }
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}
