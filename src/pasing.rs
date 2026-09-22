use std::fs;

pub fn read_file(&str: filepath){
    let contents = fs::read_to_string(filepath)
        .expect("Failed to read the file");
}
