use super::{Flag, Type};
use std::env::{args, Args};

fn get_flags(args: &mut Args) -> Vec<Flag> {
    let mut flags: Vec<Flag> = Vec::new();

    for id in 0..args.len() {
        flags.push(Flag::new(id, args));
    }

    flags
}

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

    pub fn get_flag(&self, id: usize) -> Option<&Flag> {
        self.flags.get(id)
    }

    // DONE: get flags with same type -> Vec<&Flag>
    pub fn get_by_type(&self, flag_type: Type) -> Vec<&Flag> {
        let mut flags: Vec<&Flag> = Vec::new();
        for flag in &self.flags {
            if flag.flag_type == flag_type {
                flags.push(flag);
            }
        }
        flags
    }
}
