use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path};
use std::string::FromUtf8Error;
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    paths: Vec<String>,
}

#[derive(Debug)]
pub enum CustomError {
    FileOpenError(io::Error),
    FileReadError(io::Error),
    Utf8Error(FromUtf8Error),
}

pub fn start() -> Result<(), CustomError> {
    let args = Args::parse();

    for path in args.paths {
        let content_string = get_content(path)?;
        print!("{}", content_string);
    }

    Ok(())
}

fn get_content<T: AsRef<Path>>(path: T) -> Result<String, CustomError> {
    let mut f = File::open(path).map_err(CustomError::FileOpenError)?;
    let mut buffer: Vec<u8> = Vec::new();
    let _size = f.read_to_end(&mut buffer).map_err(CustomError::FileReadError)?;
    let content_string = String::from_utf8(buffer).map_err(CustomError::Utf8Error)?;

    Ok(content_string)
}