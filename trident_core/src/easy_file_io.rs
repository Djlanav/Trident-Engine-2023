use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use glium::{DrawError, ProgramCreationError};

pub fn error_write_to_output(path: &str, error: ProgramCreationError) {
    let mut file = File::create(path).expect("Failed to create file.");

    let error_as_string = error.to_string();

    file.write(error_as_string.as_ref()).expect("File write failed.");
}

pub fn write_draw_error(path: &str, error: DrawError) {
    let mut file = File::create(path).expect("Failed to create file.");

    let error_as_string = error.to_string();

    file.write(error_as_string.as_ref()).expect("File write failed.");
}

/// Load file into memory
pub fn basic_file_load(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}