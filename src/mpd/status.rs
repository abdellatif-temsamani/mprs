use mpd::{Client, Song, Status};
use std::process::exit;

fn format_duration(dur: i64) -> String {
    format!("{}:{}", (dur / 60) % 60, dur % 60,)
}

fn current_song(song: Song) {
    println!(
        "Song: {}, duration: {}",
        song.title.unwrap_or(song.file),
        format_duration(song.duration.unwrap().num_seconds())
    );
}

fn get_status(status: Status) {
    // DONE: format status
    println!(
        "state: {:?}, volume: {}, repeat: {}\n{}",
        status.state,
        status.volume,
        status.repeat,
        format!(
            "random: {:?}, single: {:?}, consume: {:?}\ncrossfade: {:?}, queue_len: {:?}",
            status.random, status.single, status.consume, status.crossfade, status.queue_len
        )
    );
}

pub fn status(cli: &mut Client, silent: bool) {
    if silent {
        println!("[Error] -> cannot show status silent mode");
        exit(1);
    }

    current_song(cli.currentsong().unwrap().unwrap());
    // getstatus(cli.status());
    match cli.status() {
        Ok(status) => get_status(status),
        Err(err) => println!("{}", err),
    }
}
