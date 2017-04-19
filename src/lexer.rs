use pom::Parser;
use pom::parser::*;

use std::str::FromStr;


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
    term > 32 && term < 127 && term != b'#'
    //space is 32, lower is control characters
    //127 is a control character, 128 and above are UTF-8 things
}

fn utf8(v: Vec<u8>) -> String {
    String::from_utf8(v).unwrap()
}

pub fn command() -> Parser<u8, (Option<u8>, Vec<String>)> {
    let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
    let number = integer.collect().convert(String::from_utf8).convert(|s|u8::from_str(&s));
    number.opt() + (space() * word()).repeat(0..) - comment().opt()
}

#[test]
fn command_comment() {
    let input = ::pom::DataInput::new(b" \tfoo\t\t   bar#comment here");
    let output = command().parse(&mut input.clone());
    assert_eq!(output, Ok((None, vec!["foo".to_string(), "bar".to_string()])));
}

#[test]
fn command_id() {
    let input = ::pom::DataInput::new(b"12 quit");
    let output = command().parse(&mut input.clone());
    
    assert_eq!(output, Ok((Some(12), vec!["quit".to_string()])));
}