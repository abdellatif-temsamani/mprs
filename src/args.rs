use colored::*;

use crate::{client, help_menu};
use mpd::Client;
use std::{env, net::TcpStream};

pub fn argv_init(mut mpd_client: Client<TcpStream>) {
    let argv: env::Args = env::args();

    if argv.len() == 1 {
        client::info(&mut mpd_client);
    }

    for command in argv {
        match &command.to_lowercase() as &str {
            "pause" => {
                client::toggle(&mut mpd_client, String::from("pause"));
                break;
            }

            "play" => {
                client::toggle(&mut mpd_client, String::from("play"));
                break;
            }

            "stop" => {
                client::toggle(&mut mpd_client, String::from("stop"));
                break;
            }

            "toggle" => {
                client::toggle(&mut mpd_client, String::new());
                break;
            }

            "prev" => {
                client::change(&mut mpd_client, 0);
                break;
            }

            "next" => {
                client::change(&mut mpd_client, 1);
                break;
            }

            "outputs" => {
                client::outputs(mpd_client.outputs().ok().unwrap());
                break;
            }

            "help" => {
                help_menu();
                break;
            }

            "status" => {
                client::info(&mut mpd_client);
                break;
            }

            "mprs" => continue,

            _ => {
                println!("{}", "error command not found".bright_red().bold());
                help_menu();
                break;
            }
        }
    }
}
