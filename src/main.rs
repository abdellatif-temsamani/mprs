extern crate mpd;

mod args;
mod mpd_client;
mod utils;

use args::Cli;
use clap::Parser;
use mpd_client::client::{connect_client, parse_args};

fn main() {
    let argv = Cli::parse();
    let mut client = connect_client(argv.host, argv.port);
    parse_args(argv.command, &mut client);
}
