use mprs_lib::args::Argv;

extern crate mprs_lib;

fn main() {
    let args: Argv = Argv::new();
    println!("{:#?}", args.flags);
}
