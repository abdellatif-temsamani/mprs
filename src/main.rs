use std::process::exit;

use args::{Cli, Commands};
use clap::Parser;

mod args;

fn main() {
    let app = Cli::parse();
    // TODO: add mpd client

    match app.command {
        Commands::Play => {
            // TODO: play the current song with client
            println!("playing the song...");
            exit(0)
        }
        Commands::Pause => {
            // TODO: pause the current song with client
            println!("paused the song...");
            exit(0)
        }
    }
}
