use std::{fs::File, io::ErrorKind};
use std::io::prelude::*;
use std::io::Result;

fn do_something(file_name: &str) -> Result<()> {
    let create_result = File::create(file_name); 
    match create_result {
        Ok(mut file) => {
            match file.write_all(b"Hello, world!") {
                Ok(()) => return Ok(()),
                Err(e) => return Err(e)
            }
        }
        Err(e) => return Err(e)
    };
}






fn do_something2(file_name: &str) -> Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(b"Hello, world!")?;
    return Ok(());
}

fn do_something3(file_name: &str) -> Result<()> {
    File::create(file_name)?.write_all(b"Hello, world!")
}

fn main() {
    let file_name = "foo.txt";

    match do_something(file_name) {
        Ok(()) => {
            println!("Wrote to {}", file_name)
        }
        Err(e) => {
            println!("Failure writing to {}: {:?}", file_name, e);
            match e.kind() {
                ErrorKind::NotFound => println!("File not found"),
                ErrorKind::PermissionDenied => println!("Unauthorized to access file"),
                other => println!("Some other error {:?}", other)
            }
        }
    }
}