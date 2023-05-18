use mprs::{args::Argv, config::ConfigManager, mpd::MpdClient};

fn main() {
    let mut args: Argv = Argv::new();

    let mut config_manager: ConfigManager = ConfigManager::new();
    config_manager.update(args.parse_config());

    let connection: [String; 2] = config_manager.get_connection();
    let mut mpd_client: MpdClient = MpdClient::new(connection[0].clone(), connection[1].clone());
    mpd_client.connect();

    mpd_client.command(args.parse_command(), config_manager.silent())
}
