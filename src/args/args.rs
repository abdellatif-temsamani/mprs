use super::Flag;
use std::env::args;
use std::env::Args;

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
}

pub fn get_flags(args: &mut Args) -> Vec<Flag> {
    let mut flags: Vec<Flag> = Vec::new();

    for id in 0..args.len() {
        // OPTIMIZE: probably i should ignore the first arg
        flags.push(Flag::new(id, args));
    }

    flags
}
