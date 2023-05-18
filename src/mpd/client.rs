use colored::Colorize;
use mpd::{Client, State};
use std::{net::TcpStream, process::exit};

use crate::config::Param;

use super::{
    commands::{self, Queue},
    status::status,
};

/// # MpdClient
/// contains
/// - __host__: String,
/// - __port__: String,
/// - __client__: Option<Client<TcpStream>>,
#[derive(Debug)]
pub struct MpdClient {
    host: String,
    port: String,
    client: Option<Client<TcpStream>>,
}

impl MpdClient {
    /// # new
    ///
    /// new client
    ///
    /// ## params
    /// - __host__ : String
    /// - __port__ : String
    ///
    pub fn new(host: String, port: String) -> Self {
        Self {
            host,
            port,
            client: None,
        }
    }

    /// # connect
    ///
    /// connect the client to mpd server
    /// and validate it
    pub fn connect(&mut self) {
        self.client = if let Ok(client) = Client::connect(format!("{}:{}", self.host, self.port)) {
            Some(client)
        } else {
            None
        };

        self.vadidate_client();
    }

    pub fn command(&mut self, param: Param, silent: bool) {
        let cli = self.get_client().unwrap();
        match &param.value as &str {
            "play" => commands::play_pause_stop(cli, silent, State::Play),
            "pause" => commands::play_pause_stop(cli, silent, State::Pause),
            "stop" => commands::play_pause_stop(cli, silent, State::Stop),
            "toggle" => commands::toggle_client(cli, silent),
            "next" => commands::prev_next(cli, silent, Queue::Next),
            "prev" => commands::prev_next(cli, silent, Queue::Prev),
            "kill" => commands::kill_mpd(cli, silent),
            "status" | "mprs_status" => status(cli, silent),

            &_ => {
                println!(
                    "[Error] -> {} unknown command flag",
                    param.value.on_red().black()
                );
                exit(1);
            }
        }
    }

    fn get_client(&mut self) -> Option<&mut Client> {
        self.client.as_mut()
    }

    fn vadidate_client(&self) {
        if self.client.is_some() {
        } else {
            println!(
                "[Error] -> cannot find MPD running on {}:{}",
                self.host.on_red().black(),
                self.port.on_red().black()
            );
            exit(1);
        }
    }
}
