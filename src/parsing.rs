use std::fs;
use cartdridge::*;

pub fn rom_extract(&str: filepath) -> Vec<u8> {
    let contents = fs::read_to_string(filepath)
        .expect("Failed to read the file");
    contents.as_bytes();
}

pub fn rom_integrity_check(&str: filepath){
    let contents = read_file(filepath);
    // check if logo is OK
    
}
