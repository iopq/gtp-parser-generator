use pom::Parser;
use pom::parser::*;


fn space() -> Parser<u8, ()> {
    one_of(b" \t").repeat(0..).discard()
}

fn comment() -> Parser<u8, ()> {
    (sym(b'#') - none_of(b"\r\n").repeat(0..)).discard()
}

fn word() -> Parser<u8, String> {
    is_a(non_control_non_space_non_comment).repeat(1..).map(utf8)
}

fn non_control_non_space_non_comment(term: u8) -> bool {
	term > 32 && term != 127 && term != b'#'
	//control characters in ascii and space is 32
}

fn utf8(v: Vec<u8>) -> String {
	String::from_utf8(v).unwrap()
}

pub fn command() -> Parser<u8, Vec<String>> {
    (space() * word()).repeat(0..) - comment().opt()
}

#[test]
fn command_comment() {
	let input = ::pom::DataInput::new(b" \tfoo\t\t   bar#comment here");
    let output = command().parse(&mut input.clone());
	assert_eq!(output, Ok(vec!["foo".to_string(), "bar".to_string()]));
}