use colored::{ColoredString, Colorize};
use mpd::{Song, State, Status};

pub fn format_duration(dur: i64) -> String {
    format!("{}:{}", (dur / 60) % 60, dur % 60,)
}

pub fn current_song(song: Song) {
    println!(
        "Song: {}, duration: {}",
        song.title.unwrap_or(song.file).green(),
        format_duration(song.duration.unwrap().num_seconds()).green()
    );
}

pub fn format_boolstr(boolean: bool) -> ColoredString {
    match boolean {
        true => "true".green(),
        false => "false".red(),
    }
}

pub fn format_state(state: State) -> ColoredString {
    match state {
        State::Stop => "Stop".red(),
        State::Play => "Play".green(),
        State::Pause => "Pause".blue(),
    }
}

pub fn format_status(status: Status) {
    // DONE: format status
    println!(
        "state: {}, volume: {}, repeat: {}\n\nrandom: {}, single: {}, consume: {}\ncrossfade: {}, queue_len: {}",
        format_state(status.state),
        status.volume.to_string().green(),
        format_boolstr(status.repeat),
        format_boolstr(status.random),
        format_boolstr(status.single),
        format_boolstr(status.consume),
        match status.crossfade {
            Some(cross) => cross.num_seconds().to_string().blue(),
            None => "None".on_red().black(),
        },
        status.queue_len
    );
}
