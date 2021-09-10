#![allow(unused_must_use, clippy::or_fun_call)]
extern crate colored;
use colored::*;

use mpd::{Client, State};
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
                println!(
                    "{} {}",
                    "paused".bright_yellow(),
                    mpd_client
                        .currentsong()
                        .unwrap()
                        .unwrap()
                        .title
                        .unwrap()
                        .as_str()
                        .bright_yellow(),
                );
            }

            "play" => {
                mpd_client.play();
                println!(
                    "{} {}",
                    "playing".green(),
                    mpd_client
                        .currentsong()
                        .unwrap()
                        .unwrap()
                        .title
                        .unwrap()
                        .as_str()
                        .green(),
                );
            }

            "toggle" => {
                // DONE: pause when it playing
                // DONE: play when it paused or stopped
                // mpd_client
                // .unwrap()
                // .status()
                match mpd_client.status().unwrap().state {
                    State::Play => {
                        mpd_client.pause(true);
                        println!(
                            "{} {}",
                            "paused".bright_yellow(),
                            mpd_client
                                .currentsong()
                                .unwrap()
                                .unwrap()
                                .title
                                .unwrap()
                                .as_str()
                                .bright_yellow(),
                        );
                    }
                    _ => {
                        mpd_client.play();
                        println!(
                            "{} {}",
                            "playing".green(),
                            mpd_client
                                .currentsong()
                                .unwrap()
                                .unwrap()
                                .title
                                .unwrap()
                                .as_str()
                                .green(),
                        );
                    }
                }
            }

            "stop" => {
                mpd_client.stop();
                println!(
                    "{} {}",
                    "stopped".bright_red(),
                    mpd_client
                        .currentsong()
                        .unwrap()
                        .unwrap()
                        .title
                        .unwrap()
                        .as_str()
                        .bright_red(),
                );
            }

            "next" => {
                mpd_client.next();
                println!("{}", "playing the next song".blue());
            }

            "prev" => {
                mpd_client.prev();
                println!("{}", "playing the prev song".blue());
            }

            "outputs" => {
                for output in mpd_client.outputs().ok().unwrap() {
                    print!("{}: ", output.name.bright_purple());
                    if output.enabled {
                        print!("{}, ", "enabled".bright_green());
                    } else {
                        print!("{}, ", "disabled".bright_red());
                    }

                    print!("{}. ", output.id.to_string().bright_blue());
                    println!();
                }
            }

            "help" => help_menu(),

            "mprs" => continue,

            _ => {
                println!("{}", "error command not found".bright_red().bold());
                help_menu();
            }
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
