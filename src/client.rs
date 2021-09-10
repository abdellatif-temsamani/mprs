use mpd::Client;
use std::net::TcpStream;

pub fn new(host: String, port: String) -> Client<TcpStream> {
    Client::connect(format!("{}:{}", host, port)).unwrap()
}

