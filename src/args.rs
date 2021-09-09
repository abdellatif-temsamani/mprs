#![allow(unused_must_use)]

use mpd::Client;
use std::{env, net::TcpStream};

pub fn argv_init(mut mpd_client: Client<TcpStream>) {
    let argv: env::Args = env::args();

    if argv.len() == 1 {
        println!(
                    "title    : {:?}\nartist   : {:?}\nalbum    : {:?}\nduration : {:?}/{:?}\nvolume   : {:?} ",
                    mpd_client.currentsong().unwrap().unwrap().title.unwrap(),

                    mpd_client
                        .currentsong()
                        .unwrap()
                        .unwrap()
                        .tags
                        .entry("Artist".to_string())
                        .or_insert(String::new()),

                    mpd_client
                        .currentsong()
                        .unwrap()
                        .unwrap()
                        .tags
                        .entry("Album".to_string())
                        .or_insert(String::new()),

                    // TODO: convert min only to min:sec
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

                    mpd_client.status().unwrap().volume
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
                println!("playing");
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

            "status" => {
                println!(
                    "title    : {:?}\nartist   : {:?}\nalbum    : {:?}\nduration : {:?}/{:?}\nvolume   : {:?} ",
                    mpd_client.currentsong().unwrap().unwrap().title.unwrap(),

                    mpd_client
                        .currentsong()
                        .unwrap()
                        .unwrap()
                        .tags
                        .entry("Artist".to_string())
                        .or_insert(String::new()),

                    mpd_client
                        .currentsong()
                        .unwrap()
                        .unwrap()
                        .tags
                        .entry("Album".to_string())
                        .or_insert(String::new()),

                    // TODO: convert min only to min:sec
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

                    mpd_client.status().unwrap().volume
                        );
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
mprs version: 0.1.4

Commands:
    play    => play the current  song
    pause   => pause the current song
    stop    => stop the current  song
    next    => play the next song
    prev    => pause the prev song
    status   => show stats of current song
    outputs => shows outputs
    help    => shows this help menu
"
    )
}
