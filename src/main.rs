mod core;

use std::env;

use crate::core::emulator::Emulator;


const HELP_STRING: &str = "Usage: cargo run -- <ROM>";

fn main() {
    if env::args().len() != 2{
        println!("{}", HELP_STRING);
        return ;
    }
    let _emulator = Emulator::new();
}
