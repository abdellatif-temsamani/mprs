extern crate mpd;

mod args;
mod mpd_client;
mod utils;

use args::{Cli, Commands};
use clap::Parser;
use mpd_client::client::connect_client;
use mpd_client::songs::{list, list_queue};

fn main() {
    let argv = Cli::parse();
    let client = connect_client(argv.host, argv.port);

    match argv.command {
        Commands::List { path } => list(client, &path),
        Commands::Queued => list_queue(client),
    }
}
