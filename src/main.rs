use std::env;

const HELP_STRING: &str = "Usage: cargo run -- <ROM>";

fn main() {
    if env::args().len() != 2{
        println!("{}", HELP_STRING);
        return ;
    }

    check_rom_integrity(env::args[1]);
}
