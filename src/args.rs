#![allow(unused_must_use, clippy::or_fun_call)]
use colored::*;

use crate::client;
use mpd::Client;
use std::{env, net::TcpStream};

pub fn argv_init(mut mpd_client: Client<TcpStream>) {
    let argv: env::Args = env::args();

    if argv.len() == 1 {
        client::info(&mut mpd_client);
    }

    for arg in argv {
        match &arg.to_lowercase() as &str {
            "pause" => client::toggle(&mut mpd_client, String::from("pause")),

            "play" => client::toggle(&mut mpd_client, String::from("play")),

            "stop" => client::toggle(&mut mpd_client, String::from("stop")),

            "toggle" => client::toggle(&mut mpd_client, String::new()),

            "prev" => client::change(&mut mpd_client, 0),

            "next" => client::change(&mut mpd_client, 1),

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

            "status" => client::info(&mut mpd_client),

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
    {}            {} play the current  song
    {}            {} pause the current song
    {}            {} stop the current  song
    {}            {} play the next song
    {}            {} pause the prev song
    {}            {} shows outputs
    {}            {} shows this help menu
",
        "version".green(),
        "0.1.4".bright_green(),
        "no args | status".bright_blue(),
        "=>".yellow(),
        "play".bright_blue(),
        "=>".yellow(),
        "pause".bright_blue(),
        "=>".yellow(),
        "stop".bright_blue(),
        "=>".yellow(),
        "next".bright_blue(),
        "=>".yellow(),
        "prev".bright_blue(),
        "=>".yellow(),
        "outputs".bright_blue(),
        "=>".yellow(),
        "help".bright_blue(),
        "=>".yellow(),
    )
}
