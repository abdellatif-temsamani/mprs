use std::process::exit;

use crate::args::flag::Flag;

pub struct Config {
    pub flag: String,
    pub value: String,
}

pub fn parse_config(config_flags: Vec<Flag>) -> Vec<Config> {
    let mut configs: Vec<Config> = Vec::new();

    for flag in config_flags {
        configs.push(filter_config(flag));
    }

    configs
}

fn filter_config(flag: Flag) -> Config {
    let flag_values: Vec<String> = flag
        .value
        .split("=")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();

    match &flag_values[0] as &str {
        "--host" | "--port" => Config {
            flag: flag_values[0].clone(),
            value: flag_values[1].clone(),
        },

        "--silent" | "-q" => Config {
            flag: "--silent".to_owned(),
            value: "True".to_owned(),
        },

        &_ => {
            println!("[Error] -> unknown flag");
            exit(1);
        }
    }
}
