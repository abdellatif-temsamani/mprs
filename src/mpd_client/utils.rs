use std::process::exit;

use colored::Colorize;
use mpd::{Client, Song};

pub fn get_current_song(client: &mut Client) -> Song {
    match client.currentsong() {
        Ok(song) => match song {
            Some(value) => value,
            None => {
                eprintln!("{}", "[mprs]:: Current song not found".red().bold());
                exit(1);
            }
        },
        Err(_err) => {
            eprintln!("{}", "[mprs]:: Current song not found".red().bold());
            exit(1);
        }
    }
}
