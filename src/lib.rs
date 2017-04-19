#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate pom;

#[macro_use]
mod macros;

pub mod lexer;

use pom::DataInput;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {

    errors {
        NoCommand {}
        NotEnoughArguments {}
    }

    }

}

use errors::*;

#[derive(Debug, PartialEq)]
pub struct Command {
    name: String,
    id: Option<u8>,
    args: Vec<String>,
}

pub fn try_parse(input: &[u8]) -> Result<Command> {
    let input = DataInput::new(input);
    let parse = lexer::command().parse(&mut input.clone());
    let (id, mut parse) = parse.unwrap();

    let rest = parse.split_off(1);
    let ref first = parse[0];
    Ok(Command { name: first.clone(), id: id, args: rest })
}

#[test]
fn parse_quit() {
    let quit = try_parse(b"quit");
    assert_eq!(quit.unwrap(), Command{ name: "quit".to_string(), id: None, args: Vec::new() });
}