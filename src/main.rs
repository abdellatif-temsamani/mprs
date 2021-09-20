use colored::*;
use configr::Config;

mod args;
mod client;
mod config;

fn main() {
    let mpd_config = config::MpdConfig::load("mprs", false).unwrap();
    let mpd_client = client::new(mpd_config.host, mpd_config.port);
    args::argv_init(mpd_client);
}

pub fn help_menu() {
    println!(
        "
Usage: mprs <command>
mprs {}: {}

Commands:
    {} {} show stats of current song
    {}            {} play the current  song
    {}            {} pause the current song
    {}            {} stop the current  song
    {}            {} play the next song
    {}            {} pause the prev song
    {}            {} shows outputs
    {}            {} shows this help menu
",
        "version".green(),
        "0.1.4".bright_green(),
        "no args | status".bright_blue(),
        "=>".yellow(),
        "play".bright_blue(),
        "=>".yellow(),
        "pause".bright_blue(),
        "=>".yellow(),
        "stop".bright_blue(),
        "=>".yellow(),
        "next".bright_blue(),
        "=>".yellow(),
        "prev".bright_blue(),
        "=>".yellow(),
        "outputs".bright_blue(),
        "=>".yellow(),
        "help".bright_blue(),
        "=>".yellow(),
    )
}
