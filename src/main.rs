extern crate mpd;

mod args;
mod mpd_client;

use std::process::exit;

use args::{Cli, Commands};
use clap::Parser;
use mpd_client::connect_client;

fn main() -> ! {
    let argv = Cli::parse();
    let mut mpd_client = connect_client(argv.host, argv.port);

    match argv.command {
        Commands::Play => {
            let _ = mpd_client.play();
            println!("playing the current song");
            exit(0)
        }
        Commands::Pause => {
            let _ = mpd_client.pause(true);
            println!("paused the song");
            exit(0)
        }
        Commands::Kill => {
            let _ = mpd_client.kill();
            println!("mpd stopped running");
            exit(0)
        }
    }
}
