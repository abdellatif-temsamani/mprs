use super::utils::get_current_song;
use crate::utils::{err_print, print};
use colored::Colorize;
use mpd::Client;

pub fn play(client: &mut Client, quite: bool) {
    match client.play() {
        Ok(_) => {
            let song = get_current_song(client);
            if !quite {
                print(format!(
                    "{} '{}'",
                    "playing".green(),
                    song.title.unwrap().green()
                ));
            }
        }
        Err(err) => {
            err_print(err.to_string());
        }
    }
}

pub fn pause(client: &mut Client, quite: bool) {
    match client.pause(true) {
        Ok(_) => {
            let song = get_current_song(client);
            if !quite {
                print(format!(
                    "{} '{}'",
                    "Pause".green(),
                    song.title.unwrap().green()
                ))
            }
        }
        Err(err) => {
            err_print(err.to_string());
        }
    }
}

pub fn prev(client: &mut Client, quite: bool) {
    pause(client, true);
    match client.prev() {
        Ok(_) => {
            let song = get_current_song(client);
            if !quite {
                print(format!(
                    "{} '{}'",
                    "[MPRS]:: Going back to".green(),
                    song.title.unwrap().green()
                ));
            }
        }
        Err(err) => {
            err_print(err.to_string());
        }
    }
    play(client, true);
}

pub fn next(client: &mut Client, quite: bool) {
    pause(client, true);
    match client.next() {
        Ok(_) => {
            let song = get_current_song(client);
            if !quite {
                print(format!(
                    "{} '{}'",
                    "[MPRS]:: Playing next".green(),
                    song.title.unwrap().green()
                ));
            }
        }
        Err(err) => {
            err_print(err.to_string());
        }
    }

    play(client, true);
}

pub fn stop(client: &mut Client, quite: bool) {
    match client.stop() {
        Ok(_) => {
            let song = get_current_song(client);
            if !quite {
                print(format!(
                    "{} '{}'",
                    "stopped".green(),
                    song.title.unwrap().green()
                ));
            }
        }
        Err(err) => {
            err_print(err.to_string());
        }
    }
}
