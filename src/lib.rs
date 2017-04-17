macro_rules! first {
    ( $a:tt $(, $rest:tt)* ) => ( $a )
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

macro_rules! commands {

    ( $(#[$attr:meta])* enum $enumer:ident { $($i:ident => $e:tt $( ( $($m:ident),* ) )* ; )* } ) => {
        $(
             #[$attr]
        )*
        #[derive(Clone, PartialEq, Hash, Debug)]
        
        pub enum $enumer {
        $(
            $i $( ( ( $($m),*, ) ) )*,
        )*
        }
        
        impl $enumer {
            pub fn as_str(&self) -> &str {
                match *self {
                    $(
                        $enumer :: $i $( ( first!(_ $(, $m)* ) ) )* => $e,
                    )*
                }
            }
            
            pub fn from_parsed_command(parsed: Command) -> Result<Self, ParseError> {
                match parsed.name {
                    $(
                        //$e and $i level
                        $e => Ok($enumer::$i $(  ( { let mut iter = parsed.args.into_iter(); (
                            $( replace_expr!($m iter.next().unwrap().parse().unwrap()), )*
                        )   } ) )* ),
                        
                    )*
                    _ => Err(ParseError{})
                }
            }
            
            pub fn list_commands() -> String {
                [$($e),*].join("\n")
            }
        }
    };
}

pub struct Command<'a> {
    name: &'a str,
    args: Vec<&'a str>
}

#[derive(Debug)]
pub struct ParseError {

}

commands!(enum Foo { One => "two"; });

commands!(enum Bar {
    Foo => "bar" (String, String);
});

commands!(enum Baz {
   First => "kek" (i32, String);
   Second => "wow" (i32, i32);
});

commands!(enum Quux {
    Long => "long" (String);
    Short => "short";
});

#[test]
fn stringify_foo() {
    let x = Foo::One;
    assert_eq!(x.as_str(), "two");
}

#[test]
fn stringify_baz_first() {
    let x = Baz::First((3, "wew".into()));
    assert_eq!(x.as_str(), "kek");
}

#[test]
fn stringify_baz_second() {
    let x = Baz::Second((2, 3));
    assert_eq!(x.as_str(), "wow");
}

#[test]
fn stringify_quux() {
    let x = Quux::Short;
    assert_eq!(x.as_str(), "short");
}

#[test]
fn foo_from_command() {
    let x = Foo::from_parsed_command(Command{ name: "two", args: Vec::new() });
    assert_eq!(x.unwrap(), Foo::One);
}


#[test]
fn baz_from_command() {
    let x = Baz::from_parsed_command(Command{ name: "kek", args: vec!["1", "two"] });
    assert_eq!(x.unwrap(), Baz::First((1, "two".to_string())));
}

#[test]
fn foo_list_commands() {
    let x = Foo::list_commands();
    assert_eq!(x, "two".to_string());
}

#[test]
fn quux_list_commands() {
    let x = Quux::list_commands();
    assert_eq!(x, "long\nshort");
}