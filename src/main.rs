mod args;
mod client;

fn main() {
    let mpd_client = client::new("127.0.0.1", 6600);
    args::argv_init(mpd_client);
}
