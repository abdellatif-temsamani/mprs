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
    // TODO: format status  <18-07-22, abdellatif>
    println!("{:?}", status);
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
