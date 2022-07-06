use super::flag::{Flag, Type};
use std::env::{args, Args};

fn get_flags(args: &mut Args) -> Vec<Flag> {
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
    pub fn get_by_type(&self, flag_type: Type) -> Vec<&Flag> {
        // DONE: get flags with same type -> Vec<&Flag>
        let mut flags: Vec<&Flag> = Vec::new();
        for flag in &self.flags {
            if flag.flag_type == flag_type {
                flags.push(flag);
            }
        }
        flags
    }
}
