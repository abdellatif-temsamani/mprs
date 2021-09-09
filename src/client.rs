use mpd::Client;
use std::net::TcpStream;

pub fn new(host: &str, port: i32) -> Client<TcpStream> {
    Client::connect(format!("{}:{}", host, port)).unwrap()
}

