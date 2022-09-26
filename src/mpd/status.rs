use colored::Colorize;
use mpd::Client;
use std::process::exit;

use crate::format;

pub fn status(cli: &mut Client, silent: bool) {
    if silent {
        println!("[Error] -> cannot show status {}", "silent mode".on_red().black());
        exit(1);
    }

    format::current_song(cli.currentsong().unwrap().unwrap());
    // getstatus(cli.status());
    match cli.status() {
        Ok(status) => format::format_status(status),
        Err(err) => println!("{}", err),
    }
}
