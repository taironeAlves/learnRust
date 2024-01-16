use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn main() {
    if read_file_contents(PathBuf::from("src/main.rs")).is_ok() {
        println!("O arquivo existe");
    }

    if read_file_contents(PathBuf::from("non-existent-flie.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.")
    }
}

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    let mut file = match File::open(path) {
        Ok(file_handle) => file_handle,        // return
        Err(io_error) => return Err(io_error), //return
    };

    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    }

    Ok(string)
}
