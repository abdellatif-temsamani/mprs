use colored::Colorize;
use mpd::Client;

pub fn play(mut client: Client) {
    match client.play() {
        Ok(_) => match client.currentsong().unwrap() {
            Some(song) => println!("{} '{}'", "playing".green(), song.title.unwrap().green()),
            None => println!("{}", "no current song to play".red().bold()),
        },
        Err(err) => {
            println!("{:?}", err.to_string().red().bold());
        }
    }
}

pub fn pause(mut client: Client) {
    match client.pause(true) {
        Ok(_) => match client.currentsong().unwrap() {
            Some(song) => println!("{} '{}'", "paused".green(), song.title.unwrap().green()),
            None => println!("{}", "no current song to play".red().bold()),
        },
        Err(err) => {
            println!("{:?}", err.to_string().red().bold());
        }
    }
}

pub fn prev(mut client: Client) {
    client.pause(true).unwrap();
    match client.prev() {
        Ok(_) => match client.currentsong().unwrap() {
            Some(song) => println!(
                "{} '{}'",
                "going back to".green(),
                song.title.unwrap().green()
            ),
            None => println!("{}", "no current song to play".red().bold()),
        },
        Err(err) => {
            println!("{:?}", err.to_string().red().bold());
        }
    }
    client.play().unwrap();
}

//NOTE: pause and play are there
// to fix weird in mpd bug no idea why
pub fn next(mut client: Client) {
    client.pause(true).unwrap();
    match client.next() {
        Ok(_) => match client.currentsong().unwrap() {
            Some(song) => println!(
                "{} '{}'",
                "playing next".green(),
                song.title.unwrap().green()
            ),
            None => println!("{}", "no current song to play".red().bold()),
        },
        Err(err) => {
            println!("{:?}", err.to_string().red().bold());
        }
    }

    client.play().unwrap();
}

pub fn stop(mut client: Client) {
    match client.stop() {
        Ok(_) => match client.currentsong().unwrap() {
            Some(song) => println!("{} '{}'", "stopped".green(), song.title.unwrap().green()),
            None => println!("{}", "no current song to play".red().bold()),
        },
        Err(err) => {
            println!("{:?}", err.to_string().red().bold());
        }
    }
}
