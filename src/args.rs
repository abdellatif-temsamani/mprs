#![allow(unused_must_use)]

use mpd::Client;
use std::{env, net::TcpStream};

pub fn argv_init(mut mpd_client: Client<TcpStream>) {
    let argv: env::Args = env::args();

    for arg in argv {
        match &arg as &str {
            "pause" => {
                mpd_client.pause(true);
                println!("paused");
            }

            "play" => {
                println!("playing next; {:?}", mpd_client.stats());
                mpd_client.play();
                println!("playing");
            }

            "next" => {
                mpd_client.next();
            }

            "prev" => {
                mpd_client.prev();
                println!("playing");
            }
            _ => continue,
        }
    }
}
