use clap::{ColorChoice, Parser, Subcommand};
use colored::Colorize;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, required = false, default_value = "127.0.0.1")]
    pub host: String,
    #[arg(long, required = false, default_value = "6600")]
    pub port: String,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(
        color= ColorChoice::Always,
        alias = "ls",
        about = format!("{} list file in mpd music_directory", "`ls` for short.".green())
    )]
    List {
        #[arg(required = false, default_value = ".")]
        path: String,
    },
    #[command(about = "Add a song to the queue")]
    Add {
        #[arg(required = false, default_value = ".")]
        path: String,
    },
    #[command(about = "Clear the queue")]
    Queued,
    #[command(about = "List the queue")]
    Clear,
}
