use super::flag::{Flag, Type};
use super::parse;
use crate::config::Param;
use std::env::{args, Args};
use std::process::exit;

fn get_flags(args: &mut Args) -> Vec<Flag> {
    if args.len() == 1 {
        // TODO: print status
        println!("no args were given");
        exit(0)
    }

    let mut flags: Vec<Flag> = Vec::new();
    for id in 0..args.len() {
        flags.push(Flag::new(id, args));
    }
    flags
}

/// # Argv
///
/// Flag struct describe each **individual** flag
/// composed by:
/// - __flags__ : Vec<Flag> -> vector of flags: Flag
///
#[derive(Debug)]
pub struct Argv {
    pub flags: Vec<Flag>,
}

impl Argv {
    /// # new Argv
    ///
    /// Create a new Argv
    ///
    ///
    pub fn new() -> Self {
        Self {
            flags: get_flags(&mut args()),
        }
    }

    /// # get_flags
    ///
    /// Gets argments into Flag and return Option<&flag::Flag>
    ///
    /// ## Params
    /// - __id__: usize
    ///
    pub fn get_flag(&self, id: usize) -> Option<&Flag> {
        self.flags.get(id)
    }

    /// # get_by_type
    ///
    /// filters Vec<flag> by flag type return Vec<&flag::Flag>
    ///
    /// ## Params
    /// - __flag_type__: flag::Type
    ///
    pub fn get_by_type(&mut self, flag_type: Type) -> Vec<Flag> {
        // DONE: get flags with same type -> Vec<Flag>
        self.flags
            .drain_filter(|flag| flag.flag_type == flag_type)
            .collect()
    }

    pub fn parse_config(&mut self) -> Vec<Param> {
        parse::parse_config(self.get_by_type(Type::Config))
    }

    pub fn parse_command(&mut self) -> Param {
        parse::parse_command(self.get_by_type(Type::Command))
    }
}

impl Default for Argv {
    fn default() -> Self {
        Self::new()
    }
}
