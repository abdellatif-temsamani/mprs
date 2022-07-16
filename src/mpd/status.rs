use mpd::Client;
use std::process::exit;

fn current_song(cli: &mut Client) {
    let song = cli.currentsong().unwrap().unwrap();
    println!(
        "[Song]\ntitle: {}, duration: {}:{}",
        song.title.unwrap(),
        (song.duration.unwrap().num_seconds() / 60) % 60,
        song.duration.unwrap().num_seconds() % 60,
    );
}

pub fn status(cli: &mut Client, silent: bool) {
    if silent {
        println!("[Error] -> cannot show status silent mode");
        exit(1);
    }

    current_song(cli);
}
