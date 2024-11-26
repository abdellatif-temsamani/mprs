use clap::{ArgAction, Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "mprs", about = "mpd client writen in rust")]
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
    /// # controls
    #[command(about = "display info")]
    Status,
    #[command(about = "Play the queued song")]
    Play {
        #[arg(long, required = false, action = ArgAction::SetTrue)]
        quite: bool,
    },
    #[command(about = "Pause the queued song")]
    Pause {
        #[arg(long, required = false, action = ArgAction::SetTrue)]
        quite: bool,
    },
    #[command(about = "Play the next queued song")]
    Next {
        #[arg(long, required = false, action = ArgAction::SetTrue)]
        quite: bool,
    },
    #[command(about = "Play the prev queued song")]
    Prev {
        #[arg(long, required = false, action = ArgAction::SetTrue)]
        quite: bool,
    },
    #[command(about = "Stop the queued song")]
    Stop {
        #[arg(long, required = false, action = ArgAction::SetTrue)]
        quite: bool,
    },
    #[command(about = "kill MPD process")]
    Kill {
        #[arg(long, required = false, action = ArgAction::SetTrue)]
        quite: bool,
    },

    /// # Files
    #[command(
        alias = "ls",
        about = "`ls` for short. list file in mpd music_directory"
    )]
    List {
        #[arg(required = false, default_value = ".")]
        path: String,
    },

    /// # Queue
    #[command(about = "Add a song to the queue")]
    Add {
        #[arg(required = false, default_value = ".")]
        path: String,
    },
    #[command(about = "Clear the queue")]
    Queued,
    #[command(about = "List the queue")]
    Clear {
        #[arg(long, required = false, action = ArgAction::SetTrue)]
        quite: bool,
    },
}
