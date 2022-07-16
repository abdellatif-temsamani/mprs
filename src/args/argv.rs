use super::flag::{Flag, Type};
use super::parse;
use crate::config::Param;
use std::env::{args, Args};

fn get_flags(args: &mut Args) -> Vec<Flag> {
    let mut flags: Vec<Flag> = Vec::new();
    // DONE: ignore the name of command
    if args.len() >= 1 {
        flags.push(Flag::new("mprs_status".to_owned()));
    }

    args.next();

    for arg in args {
        flags.push(Flag::new(arg));
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
