use std::io::Read;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub enum FileError<'a> {
    NotFound(&'a Path),
    CouldNotRead(&'a Path),
    CouldNotParse(&'a Path), 
}

pub fn read_file(path: &Path) -> Result<String, FileError> {
    let mut s = String::new();

    match File::open(path) {
        Ok(mut f) => {
            match f.read_to_string(&mut s) {
                Ok(_)  => Ok(s),
                Err(_) => Err(FileError::CouldNotRead(path))
            }
        },
        Err(_) => Err(FileError::NotFound(path))
    }
}
