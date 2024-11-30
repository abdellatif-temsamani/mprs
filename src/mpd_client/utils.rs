use std::process::exit;

use crate::utils::err_print;
use mpd::{Client, Song};

pub fn get_current_song(client: &mut Client) -> Song {
    match client.currentsong() {
        Ok(song) => match song {
            Some(value) => value,
            None => {
                err_print("Current song not found");
                exit(1);
            }
        },
        Err(_err) => {
            err_print("Current song not found");
            exit(1);
        }
    }
}
