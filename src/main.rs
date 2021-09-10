use configr::Config;

mod args;
mod client;
mod config;

fn main() {
    let mpd_config = config::MpdConfig::load("mprs", true).unwrap();
    let mpd_client = client::new(mpd_config.host, mpd_config.port);
    args::argv_init(mpd_client);
}
