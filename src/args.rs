#![allow(unused_must_use)]
extern crate colored;
use colored::*;

use mpd::Client;
use std::{env, net::TcpStream};

pub fn argv_init(mut mpd_client: Client<TcpStream>) {
    let argv: env::Args = env::args();

    if argv.len() == 1 {
        println!(
            "{}{} {}\n{}{} {}\n{}{} {}\n{}{} {:?}/{:?}\n{}{} {:?} ",
            "title    ".bright_green().bold(),
            ":".bright_magenta(),
            mpd_client
                .currentsong()
                .unwrap()
                .unwrap()
                .title
                .unwrap()
                .as_str()
                .bright_yellow(),
            "artist   ".bright_green().bold(),
            ":".bright_magenta(),
            mpd_client
                .currentsong()
                .unwrap()
                .unwrap()
                .tags
                .entry("Artist".to_string())
                .or_insert(String::new())
                .as_str()
                .bright_purple(),
            "album    ".bright_green().bold(),
            ":".bright_magenta(),
            mpd_client
                .currentsong()
                .unwrap()
                .unwrap()
                .tags
                .entry("Album".to_string())
                .or_insert(String::new())
                .as_str()
                .cyan(),
            "duration ".bright_green().bold(),
            ":".bright_magenta(),
            // TODO: convert sec to min
            mpd_client
                .status()
                .unwrap()
                .time
                .unwrap()
                .0
                .to_std()
                .unwrap(),
            mpd_client
                .status()
                .unwrap()
                .time
                .unwrap()
                .1
                .to_std()
                .unwrap(),
            "volume   ".bright_green().bold(),
            ":".bright_magenta(),
            mpd_client.status().unwrap().volume,
        );
    }

    for arg in argv {
        match &arg.to_lowercase() as &str {
            "pause" => {
                mpd_client.pause(true);
                println!("paused");
            }

            "play" => {
                mpd_client.play();
                println!(
                    "playing {:?}",
                    mpd_client.currentsong().unwrap().unwrap().title.unwrap(),
                );
            }

            "next" => {
                mpd_client.next();
                println!("goint to the next");
            }

            "prev" => {
                mpd_client.prev();
                println!("goint to the prev");
            }

            "stop" => {
                mpd_client.stop();
                println!("mpd stopped");
            }

            "outputs" => {
                for output in mpd_client.outputs().ok().unwrap() {
                    println!("{:?}", output);
                }
            }

            "help" => help_menu(),

            _ => continue,
        }
    }
}

fn help_menu() {
    println!(
        "
Usage: mprs <command>
mprs {}: {}

Commands:
    {} {} show stats of current song
    {} {} play the current  song
    {} {} pause the current song
    {} {} stop the current  song
    {} {} play the next song
    {} {} pause the prev song
    {} {} shows outputs
    {} {} shows this help menu
",
        "version".green(),
        "0.1.4".bright_green(),
        "no args ".bright_blue(),
        "=>".yellow(),
        "play    ".bright_blue(),
        "=>".yellow(),
        "pause   ".bright_blue(),
        "=>".yellow(),
        "stop    ".bright_blue(),
        "=>".yellow(),
        "next    ".bright_blue(),
        "=>".yellow(),
        "prev    ".bright_blue(),
        "=>".yellow(),
        "outputs ".bright_blue(),
        "=>".yellow(),
        "help    ".bright_blue(),
        "=>".yellow(),
    )
}
