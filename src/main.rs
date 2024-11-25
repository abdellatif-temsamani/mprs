extern crate mpd;

use colored::Colorize;
mod args;
mod mpd_client;
mod utils;

use args::{Cli, Commands};
use clap::Parser;
use mpd_client::client::connect_client;
use mpd_client::songs::{add_to_queue, list, list_queue};

fn main() {
    let argv = Cli::parse();
    let mut client = connect_client(argv.host, argv.port);

    match argv.command {
        Commands::List { path } => list(client, &path),
        Commands::Add { path } => add_to_queue(&mut client, &path),
        Commands::Queued => list_queue(client),
        Commands::Clear => {
            let _ = client.clear().unwrap();
            println!("{}", "cleared queue".green());
        }
    }
}
