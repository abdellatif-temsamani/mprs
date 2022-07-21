use std::process::exit;

use crate::{args::flag::Flag, config::Param};

pub fn parse_command(commands_flags: Vec<Flag>) -> Param {
    if commands_flags.len() == 1 {
        return Param::command(commands_flags[0].value.clone());
    }
    {
        if commands_flags[0].value == *"mprs_status" {
            return Param::command(commands_flags[1].value.clone());
        } else {
            println!("[Error] -> cannot run more than 1 command");
            exit(1);
        }
    }
}
