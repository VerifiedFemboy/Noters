use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub fn get_file_contents(directory: PathBuf, filename: &str) -> io::Result<(String, PathBuf)> {
    let mut file = File::open(directory.join(filename))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok((content, directory))
}

pub fn write_to_file(file: File, content: &str) -> io::Result<()> {
    let mut file = file;
    file.set_len(0)?;
    file.write_all(content.as_bytes())
}

pub fn create_file_and_write(directory: PathBuf, filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(directory.join(filename))?;
    file.write_all(content.as_bytes())
}