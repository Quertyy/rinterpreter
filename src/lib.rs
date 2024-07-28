mod errors;
mod token_type;
mod scanner;
mod token;
use errors::*;
use std::path::Path;



pub fn run_file(path: &Path) -> Result<(), FileError> {
    let bytes = std::fs::read(path)?;
    run(&bytes);
    Ok(())
}

fn run(bytes: &Vec<u8>) {

}
