use mprs_lib::mpd::MpdClient;

extern crate mprs_lib;

fn main() {
    let mut cllie: MpdClient = MpdClient::new("127.0.0.1".to_owned(), "6600".to_owned());
    cllie.connect();
}
