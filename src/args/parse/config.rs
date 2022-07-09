use std::process::exit;

use crate::{args::flag::Flag, config::Param};

pub fn parse_config(config_flags: Vec<Flag>) -> Vec<Param> {
    let mut configs: Vec<Param> = Vec::new();

    for flag in config_flags {
        configs.push(filter_config(flag));
    }

    configs
}

fn filter_config(flag: Flag) -> Param {
    let flag_values: Vec<String> = flag
        .value
        .split("=")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();

    match &flag_values[0] as &str {
        "--host" | "--port" => Param::config(flag_values[0].clone(), flag_values[1].clone()),
        "--silent" | "-q" => Param::config("--silent".to_owned(), "True".to_owned()),
        &_ => {
            println!("[Error] -> unknown config flag");
            exit(1);
        }
    }
}
