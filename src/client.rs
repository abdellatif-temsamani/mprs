#[allow(unused_mut)]
use colored::*;
use mpd::{Client, Output, State};
use std::net::TcpStream;

pub fn new(host: String, port: String) -> Client<TcpStream> {
    Client::connect(format!("{}:{}", host, port)).unwrap()
}

pub fn info(mpd_client: &mut Client<TcpStream>) {
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

pub fn toggle(mpd_client: &mut Client<TcpStream>, arg: String) {
    match arg.as_str() {
        "pause" => {
            mpd_client.pause(true).expect("error pausing");
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
            mpd_client.play().expect("error playing");
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
        "stop" => {
            mpd_client.stop().expect("error stopping");
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

        _ => {
            if mpd_client.status().unwrap().state == State::Play {
                mpd_client.pause(true).expect("error pausing");
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
            } else {
                mpd_client.play().expect("error playing");
            }
        }
    }
}

pub fn change(mpd_client: &mut Client<TcpStream>, arg: i32) {
    match arg {
        0 => {
            mpd_client.prev().expect("error jumping to the prev song");
            println!("{}", "playing the prev song".blue());
        }

        _ => {
            mpd_client.next().expect("error jumping to the next song");
            println!("{}", "playing the next song".blue());
        }
    }
}

pub fn outputs(outputs: Vec<Output>) {
    for output in outputs {
        print!("{}: ", output.name.bright_purple());
        if output.enabled {
            print!("{}, ", "enabled".bright_green());
        } else {
            print!("{}, ", "disabled".bright_red());
        }

        println!("{}. ", output.id.to_string().bright_blue());
    }
}
