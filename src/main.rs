extern crate mpd;

use colored::Colorize;
mod args;
mod mpd_client;
mod utils;

use args::{Cli, Commands};
use clap::Parser;
use mpd_client::client::connect_client;
use mpd_client::controls::{next, pause, play, prev, stop};
use mpd_client::songs::{add_to_queue, current, list, list_queue};
use utils::{err_print, print};

fn main() {
    let argv = Cli::parse();
    let mut client = connect_client(argv.host, argv.port);

    match argv.command {
        Commands::Current => current(&mut client),
        Commands::Play { quite } => play(&mut client, quite),
        Commands::Pause { quite } => pause(&mut client, quite),
        Commands::Stop { quite } => stop(&mut client, quite),
        Commands::Next { quite } => next(&mut client, quite),
        Commands::Prev { quite } => prev(&mut client, quite),
        Commands::Kill { quite } => {
            client.stop().unwrap();
            let _ = client.kill();
            if !quite {
                err_print(format!("{}", "kill MPD process".red().bold()));
            }
        }
        Commands::List { path } => list(client, &path),
        Commands::Add { path } => {
            add_to_queue(&mut client, &path);
            print(format!("{}", "added to the queue".green()));
        }
        Commands::Queued => list_queue(client),
        Commands::Clear { quite } => {
            client.clear().unwrap();
            if !quite {
                print(format!("{}", "cleared queue".green()));
            }
        }
    }
}
