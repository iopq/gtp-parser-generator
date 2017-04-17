extern crate pom;

#[macro_use]
mod macros;

pub mod lexer;

use pom::DataInput;

#[derive(Debug, PartialEq)]
pub struct Command {
    name: String,
    args: Vec<String>
}

#[derive(Debug, PartialEq)]
pub struct ParseError {

}

commands!(enum Gtp { Quit => "quit"; });

pub fn try_parse(input: &[u8]) -> Result<Command, ParseError> {
	let input = DataInput::new(input);
    let parse = lexer::command().parse(&mut input.clone());
	let mut parse = parse.unwrap();

	let rest = parse.split_off(1);
	let ref first = parse[0];
	Ok(Command { name: first.clone(), args: rest })
}

#[test]
fn parse_quit() {
    let quit = try_parse(b"quit");
	assert_eq!(quit, Ok(Command{ name: "quit".to_string(), args: Vec::new() }));
}