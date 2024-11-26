use colored::Colorize;
use mpd::Client;

pub fn play(client: &mut Client, quite: bool) {
    match client.play() {
        Ok(_) => match client.currentsong().unwrap() {
            Some(song) => {
                if !quite {
                    println!("{} '{}'", "playing".green(), song.title.unwrap().green());
                }
            }
            None => println!("{}", "no current song to play".red().bold()),
        },
        Err(err) => {
            println!("{:?}", err.to_string().red().bold());
        }
    }
}

pub fn pause(client: &mut Client, quite: bool) {
    match client.pause(true) {
        Ok(_) => match client.currentsong().unwrap() {
            Some(song) => {
                if !quite {
                    println!("{} '{}'", "paused".green(), song.title.unwrap().green());
                }
            }
            None => println!("{}", "no current song to play".red().bold()),
        },
        Err(err) => {
            println!("{:?}", err.to_string().red().bold());
        }
    }
}

pub fn prev(mut client: Client, quite: bool) {
    pause(&mut client, quite);
    match client.prev() {
        Ok(_) => match client.currentsong().unwrap() {
            Some(song) => {
                if !quite {
                    println!(
                        "{} '{}'",
                        "going back to".green(),
                        song.title.unwrap().green()
                    );
                }
            }
            None => println!("{}", "no current song to play".red().bold()),
        },
        Err(err) => {
            println!("{:?}", err.to_string().red().bold());
        }
    }
    play(&mut client, quite);
}

pub fn next(mut client: Client, quite: bool) {
    pause(&mut client, quite);
    match client.next() {
        Ok(_) => match client.currentsong().unwrap() {
            Some(song) => {
                if !quite {
                    println!(
                        "{} '{}'",
                        "playing next".green(),
                        song.title.unwrap().green()
                    )
                }
            }
            None => println!("{}", "no current song to play".red().bold()),
        },
        Err(err) => {
            println!("{:?}", err.to_string().red().bold());
        }
    }

    play(&mut client, quite);
}

pub fn stop(mut client: Client, quite: bool) {
    match client.stop() {
        Ok(_) => match client.currentsong().unwrap() {
            Some(song) => {
                if !quite {
                    println!("{} '{}'", "stopped".green(), song.title.unwrap().green());
                }
            }
            None => println!("{}", "no current song to play".red().bold()),
        },
        Err(err) => {
            println!("{:?}", err.to_string().red().bold());
        }
    }
}
