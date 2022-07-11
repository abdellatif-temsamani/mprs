use mprs_lib::{args::Argv, config::ConfigManager, mpd::MpdClient};

extern crate mprs_lib;

fn main() {
    let mut args: Argv = Argv::new();

    let mut config_manager: ConfigManager = ConfigManager::new();
    config_manager.update(args.parse_config());

    let mut mpd_client: MpdClient = MpdClient::new(config_manager.host, config_manager.port);
    mpd_client.connect();

    let _ = mpd_client.command(args.parse_command(), config_manager.silent);
}
