use std::process::exit;

use colored::Colorize;

use crate::{
    args::flag::Flag,
    config::Param,
    help::{get_vertion, help},
};

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
        .replace('"', "")
        .replace('"', "")
        .split('=')
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();

    match &flag_values[0] as &str {
        "-q" | "--silent" => Param::config("--silent".to_owned(), "True".to_owned()),
        "--host" | "--port" => Param::config(flag_values[0].clone(), flag_values[1].clone()),

        "--version" | "-v" => {
            get_vertion();
            exit(0);
        }

        "--help" | "-h" => {
            help();
            exit(0);
        }

        &_ => {
            println!(
                "[Error] -> {} unknown config flag",
                flag_values[0].on_red().black()
            );
            exit(1);
        }
    }
}
