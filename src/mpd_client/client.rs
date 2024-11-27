use mpd::Client;
use std::process::exit;

use colored::Colorize;

use super::controls::{next, pause, play, prev, stop};
use super::songs::{add_to_queue, current, list, list_queue};
use crate::args::Commands;
use crate::utils::{err_print, print};

pub fn connect_client(host: String, port: String) -> Client {
    match Client::connect(format!("{}:{}", host, port)) {
        Ok(value) => value,
        Err(_err) => {
            err_print("Connection refused");
            exit(1)
        }
    }
}

pub fn parse_args(commands: Commands, client: &mut Client) {
    match commands {
        Commands::Current => current(client),
        Commands::Play { quite } => play(client, quite),
        Commands::Pause { quite } => pause(client, quite),
        Commands::Stop { quite } => stop(client, quite),
        Commands::Next { quite } => next(client, quite),
        Commands::Prev { quite } => prev(client, quite),
        Commands::Kill { quite } => {
            client.stop().unwrap();
            let _ = client.kill();
            if !quite {
                err_print("kill MPD process");
            }
        }
        Commands::List { path } => list(client, &path),
        Commands::Add { path } => {
            add_to_queue(client, &path);
            print("added to the queue");
        }
        Commands::Queued => list_queue(client),
        Commands::Clear { quite } => {
            client.clear().unwrap();
            if !quite {
                print("cleared queue");
            }
        }
        Commands::Repeat { value, quite } => {
            client.repeat(value == "on").unwrap();
            if !quite {
                print(&format!("{} {}", "Repeat mode is".green(), value.green()));
            }
        }
        Commands::Random { value, quite } => {
            client.repeat(value == "on").unwrap();
            if !quite {
                print(&format!("{} {}", "Repeat mode is".green(), value.green()));
            }
        }
        Commands::Single { value, quite } => {
            client.repeat(value == "on").unwrap();
            if !quite {
                print(&format!("{} {}", "Repeat mode is".green(), value.green()));
            }
        }
        Commands::Consume { value, quite } => {
            client.repeat(value == "on").unwrap();
            if !quite {
                print(&format!("{} {}", "Repeat mode is".green(), value.green()));
            }
        }
    }
}
