use mpd::{status::State, Client};
use std::{net::TcpStream, process::exit};

use crate::config::Param;

/// # MpdClient
/// contains
/// - __host__: String,
/// - __port__: String,
/// - __client__: Option<Client<TcpStream>>,
#[derive(Debug)]
pub struct MpdClient {
    pub host: String,
    pub port: String,
    pub client: Option<Client<TcpStream>>,
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
        // TODO: convert commands to enum
        match &param.value as &str {
            "play" => {
                cli.play().unwrap();
                if !silent {
                    println!("[MPRS] -> playing the song");
                }
            }

            "pause" => {
                cli.pause(true).unwrap();
                if !silent {
                    println!("[MPRS] -> Pausing the song");
                }
            }

            "stop" => {
                cli.stop().unwrap();
                if !silent {
                    println!("[MPRS] -> Stoping the song");
                }
            }

            "toggle" => {
                match cli.status().unwrap().state {
                    State::Stop | State::Pause => {
                        cli.play().unwrap();
                        if !silent {
                            println!("[MPRS] -> playing the song");
                        }
                    }
                    State::Play => {
                        cli.pause(true).unwrap();
                        if !silent {
                            println!("[MPRS] -> Pausing the song");
                        }
                    }
                };
            }

            "next" => {
                cli.next().unwrap();
                if !silent {
                    println!("[MPRS] -> movin to the next");
                }
            }

            "prev" => {
                cli.prev().unwrap();
                if !silent {
                    println!("[MPRS] -> movin to the next");
                }
            }

            &_ => {
                println!("[Error] -> unknown command flag");
                exit(1);
            }
        }
    }

    fn get_client(&mut self) -> Option<&mut Client> {
        self.client.as_mut()
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
