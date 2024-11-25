use clap::{Parser, Subcommand};

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
    #[command(alias = "ls", about = "or `ls` for short")]
    List {
        #[arg(
            required = false,
            default_value = ".",
            help = "path to the files (base dir = music_directory in mpd.conf)"
        )]
        path: String,
    },
    #[command(about = "list the queue")]
    Queued,
}
