use std::process::exit;

use colored::Colorize;

use crate::{args::flag::Flag, config::Param};

pub fn parse_command(commands_flags: Vec<Flag>) -> Param {
    if commands_flags.len() >= 3 {
        println!(
            "[Error] -> cannot run more than {} command",
            "1".bright_red()
        );
        exit(1);
    } else if commands_flags.len() == 1 {
        return Param::command(commands_flags[0].value.clone());
    } else {
        return Param::command(commands_flags[1].value.clone());
    }
}
