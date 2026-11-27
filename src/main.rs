mod core;

use std::env;

use crate::core::emulator::Emulator;


const HELP_STRING: &str = "Usage: cargo run -- <ROM>";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::args().len() != 2{
        println!("{}", HELP_STRING);
        return Ok(());
    }
    let mut emulator = Emulator::new();
    let arguments: Vec<String> = env::args().collect();
    emulator.load(&arguments[1]);
    loop{
        emulator.step()?
    }
}
