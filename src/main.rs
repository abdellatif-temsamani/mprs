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
        Commands::Play { quite } => play(&mut client, quite),
        Commands::Pause { quite } => pause(&mut client, quite),
        Commands::Stop { quite } => stop(client, quite),
        Commands::Next { quite } => next(client, quite),
        Commands::Prev { quite } => prev(client, quite),
        Commands::Kill { quite } => {
            let _ = client.kill();
            if !quite {
                println!("{}", "kill MPD process".red().bold());
            }
        }
        Commands::List { path } => list(client, &path),
        Commands::Add { path } => {
            add_to_queue(&mut client, &path);
            println!("{}", "added to the queue".green());
        }
        Commands::Queued => list_queue(client),
        Commands::Clear { quite } => {
            client.clear().unwrap();
            if !quite {
                println!("{}", "cleared queue".green());
            }
        }
    }
}
