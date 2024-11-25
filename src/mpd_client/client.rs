use colored::Colorize;
use mpd::Client;
use std::process::exit;

pub fn connect_client(host: String, port: String) -> Client {
    match Client::connect(format!("{}:{}", host, port)) {
        Ok(value) => value,
        Err(_err) => {
            println!("{}", "Connection refused".red());
            exit(1)
        }
    }
}
