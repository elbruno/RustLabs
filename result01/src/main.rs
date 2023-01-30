use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> String {
    let mut file_content = String::new();

    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return "io_error file open".to_string(),
    };

    match file.read_to_string(&mut file_content) {
        Ok(_) => (),
        Err(io_error) => return "io_error file read".to_string(),
    };

    file_content
}

fn main() {
    let file_path = "data2.txt";
    let file_content = read_file_contents(PathBuf::from(file_path));
    println!("{}", file_content);
}
