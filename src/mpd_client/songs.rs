use std::process::exit;

use crate::utils::format_duration;
use colored::Colorize;
use mpd::Client;

pub fn list(mut client: Client, path: &str) {
    let dirs = client.listfiles(path);

    match dirs {
        Ok(dir) => dir.into_iter().for_each(|a| {
            if a.0 == "directory" {
                println!("{}", a.1.blue());
            }
            if a.0 == "file" {
                println!("{}", a.1.white());
            }
        }),
        Err(_err) => {
            println!("No such file or directory");
            exit(2);
        }
    }
}

pub fn list_queue(mut client: Client) {
    let queue = client.queue();
    match queue {
        Ok(songs) => {
            for song in songs {
                println!(
                    "{} {} {}",
                    song.artist.unwrap_or("no artist found".to_string()).green(),
                    format_duration(song.duration.unwrap().as_secs()),
                    song.title.unwrap_or("no title found".to_string()).blue(),
                );
            }
        }

        Err(_err) => {
            println!("error reading the queue");
            exit(1);
        }
    }
}
