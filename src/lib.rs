mod errors;
mod token_type;
mod scanner;
mod token;
mod ast;
mod ast_printer;
mod parser;
use errors::*;
use std::path::Path;
use ast::*;


pub fn run_file(path: &Path) -> Result<(), FileError> {
    let bytes = std::fs::read(path)?;
    run(&bytes);
    Ok(())
}

fn run(bytes: &Vec<u8>) {

}
