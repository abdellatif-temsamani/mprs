#![allow(unused_must_use)]

use mpd::Client;
use std::{env, net::TcpStream};

pub fn argv_init(mut mpd_client: Client<TcpStream>) {
    let argv: env::Args = env::args();

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

            "outputs" => {
                println!("{:?}", mpd_client.outputs().ok().unwrap());
            }

            "stats" => println!("{:?}", mpd_client.stats().ok().unwrap()),

            "-h" | "--help" | "help" => help_menu(),

            _ => continue,
        }
    }
}

fn help_menu() {
    println!(
        "
mprs help:
    play  => play the current
    pause => pause the current
    next  => play the next song
    prev  => pause the prev song
    stats => show stats of current song
    outputs => shows outputs
    -h | --help | help  => shows this help menu
"
    )
}

