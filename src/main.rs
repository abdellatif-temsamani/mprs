use mprs_lib::args::{Argv, Type};

extern crate mprs_lib;

fn main() {
    let args: Argv = Argv::new();
    // println!("{:#?}", args.flags);
    println!("{:#?}", args.get_by_type(Type::Config));
}
