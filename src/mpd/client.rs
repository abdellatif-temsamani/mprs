use mpd::Client;
use std::{net::TcpStream, process::exit};

#[derive(Debug)]
pub struct MpdClient {
    pub host: String,
    pub port: String,
    pub client: Option<Client<TcpStream>>,
}

impl MpdClient {
    pub fn new(host: String, port: String) -> Self {
        Self {
            host,
            port,
            client: None,
        }
    }

    pub fn connect(&mut self) {
        self.client = if let Ok(client) = Client::connect(format!("{}:{}", self.host, self.port)) {
            Some(client)
        } else {
            None
        };

        self.vadidate_client();
    }

    fn vadidate_client(&self) {
        if self.client.is_some() {
            return;
        } else {
            println!(
                "[Error] -> cannot find MPD running on {}:{}",
                self.host, self.port
            );
            exit(1);
        }
    }
}
