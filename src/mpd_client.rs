use mpd::Client;

pub fn connect_client(host: String, port: String) -> Client {
    Client::connect(format!("{}:{}", host, port)).unwrap()
}
