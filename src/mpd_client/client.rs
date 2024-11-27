use crate::utils::err_print;
use mpd::Client;
use std::process::exit;

pub fn connect_client(host: String, port: String) -> Client {
    match Client::connect(format!("{}:{}", host, port)) {
        Ok(value) => value,
        Err(_err) => {
            err_print("Connection refused".to_owned());
            exit(1)
        }
    }
}
