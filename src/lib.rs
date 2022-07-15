#![crate_type = "lib"]
#![feature(drain_filter)]

#[macro_use]
extern crate prettytable;

pub mod args;
pub mod config;
pub mod help;
pub mod mpd;
