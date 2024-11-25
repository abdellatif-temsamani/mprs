extern crate mpd;

use colored::Colorize;
mod args;
mod mpd_client;
mod utils;

use args::{Cli, Commands};
use clap::Parser;
use mpd_client::client::connect_client;
use mpd_client::controls::{next, pause, play, prev, stop};
use mpd_client::songs::{add_to_queue, list, list_queue, status};

fn main() {
    let argv = Cli::parse();
    let mut client = connect_client(argv.host, argv.port);

    match argv.command {
        Commands::Status => status(client),
        Commands::Play => play(client),
        Commands::Pause => pause(client),
        Commands::Stop => stop(client),
        Commands::Next => next(client),
        Commands::Prev => prev(client),
        Commands::Kill => {
            let _ = client.kill();
            println!("{}", "kill MPD process".red().bold());
        }
        Commands::List { path } => list(client, &path),
        Commands::Add { path } => {
            add_to_queue(&mut client, &path);
            println!("{}", "added to the queue".green());
        }
        Commands::Queued => list_queue(client),
        Commands::Clear => {
            client.clear().unwrap();
            println!("{}", "cleared queue".green());
        }
    }
}
