use std::process::exit;

use crate::utils::format_duration;
use colored::Colorize;
use mpd::{Client, Song};

pub fn list(mut client: Client, path: &str) {
    let dirs = client.listfiles(path);

    match dirs {
        Ok(dir) => dir.into_iter().for_each(|item| {
            if item.0 == "directory" {
                println!("{}", item.1.blue());
            }
            if item.0 == "file" {
                println!("{}", item.1.white());
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
            if songs.is_empty() {
                println!("{}", "Queue is empty".yellow());
            } else {
                for song in songs {
                    println!(
                        "{} {} {}",
                        song.artist.unwrap_or("no artist found".to_string()).green(),
                        format_duration(song.duration.unwrap().as_secs()),
                        song.title.unwrap_or("no title found".to_string()).blue(),
                    );
                }
            }
        }

        Err(_err) => {
            println!("error reading the queue");
            exit(1);
        }
    }
}

pub fn add_to_queue(client: &mut Client, base_dir: &str) {
    let dirs = client.listfiles(base_dir);

    match dirs {
        Ok(dir) => dir.into_iter().for_each(|item| {
            if item.0 == "directory" {
                add_to_queue(client, &item.1);
            }
            if item.0 == "file" {
                let song = Song {
                    file: format!("{}/{}", base_dir, item.1),
                    name: None,
                    title: None,
                    last_mod: None,
                    artist: None,
                    duration: None,
                    place: None,
                    range: None,
                    tags: Vec::new(),
                };
                let _ = client.push(song).unwrap();
            }
        }),
        Err(_err) => {
            println!("No such file or directory");
            exit(2);
        }
    }
}

pub fn status(mut client: Client) {
    match client.currentsong().unwrap() {
        Some(song) => {
            println!();
            println!("{}", song.file);
            println!(
                "playing '{}' by {}",
                song.title.unwrap_or("title not found".to_owned()).green(),
                song.artist.unwrap_or("artist not found".to_owned()).green(),
            );
            println!(
                "duration '{}' {}",
                format_duration(song.duration.unwrap().as_secs()).green(),
                song.file.green(),
            );
        }
        None => println!("{}", "no current song to play".red().bold()),
    };
}
