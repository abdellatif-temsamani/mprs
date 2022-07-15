use mpd::{Client, State};
use std::process::{self, exit, Command};

#[derive(Debug)]
pub enum Queue {
    Prev,
    Next,
}

pub fn toggle_client(cli: &mut Client, silent: bool) {
    match cli.status().unwrap().state {
        State::Stop | State::Pause => play_pause_stop(cli, silent, State::Play),
        State::Play => play_pause_stop(cli, silent, State::Pause),
    };
}

pub fn play_pause_stop(cli: &mut Client, silent: bool, state: State) {
    if !silent {
        println!("[MPRS] -> {:?}", state);
    }
    match state {
        State::Stop => cli.stop().unwrap(),
        State::Pause => cli.pause(true).unwrap(),
        State::Play => cli.play().unwrap(),
    };
}

pub fn prev_next(cli: &mut Client, silent: bool, next_prev: Queue) {
    if !silent {
        println!("[MPRS] -> {:?} song", next_prev);
    }

    match next_prev {
        Queue::Prev => cli.prev().unwrap(),
        Queue::Next => cli.next().unwrap(),
    };
}

pub fn kill_mpd(silent: bool) {
    if check_mpd().status.success() {
        execute("sh", "killall mpd", "[Error] -> cannot kill mpd");
    }
    if !silent {
        println!("[MPRS] -> killed mpd");
    }
}

fn check_mpd() -> process::Output {
    if cfg!(target_os = "windows") {
        println!("not support on windows Yet");
        exit(1);
    } else {
        execute("sh", "pgrep mpd", "[Error] -> did not find mpd running")
    }
}

fn execute(shell: &str, arg: &str, expection: &str) -> process::Output {
    Command::new(shell)
        .arg("-c")
        .arg(arg)
        .output()
        .expect(expection)
}
