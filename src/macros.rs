#[cfg(test)]
use errors::*;

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
            pub fn to_string(&self) -> &str {
                match *self {
                    $(
                        $enumer :: $i $( ( first!(_ $(, $m)* ) ) )* => $e,
                    )*
                }
            }
            
            pub fn from_parsed_command(parsed: $crate::Command) -> Result<Self> {
                match parsed.name.as_str() {
                    $(
                        //$e and $i level
                        $e => Ok($enumer::$i $(  ( { let mut iter = parsed.args.into_iter(); (
                            $( replace_expr!($m iter.next().unwrap().parse().unwrap()), )*
                        )   } ) )* ),
                        
                    )*
                    _ => Err(ErrorKind::NoCommand.into())
                }
            }
            
            pub fn list_commands() -> String {
                [$($e),*].join("\n")
            }
        }
    };
}

#[cfg(test)]
commands!(enum Foo { One => "two"; });

#[cfg(test)]
commands!(enum Bar {
    Foo => "bar" (String, String);
});

#[cfg(test)]
commands!(enum Baz {
   First => "kek" (i32, String);
   Second => "wow" (i32, i32);
});

#[cfg(test)]
commands!(enum Quux {
    Long => "long" (String);
    Short => "short";
});

#[test]
fn stringify_foo() {
    let x = Foo::One;
    assert_eq!(x.to_string(), "two");
}

#[test]
fn stringify_bar() {
    let x = Bar::Foo(("3".to_string(), "wew".into()));
    assert_eq!(x.to_string(), "bar");
}

#[test]
fn stringify_baz_first() {
    let x = Baz::First((3, "wew".into()));
    assert_eq!(x.to_string(), "kek");
}

#[test]
fn stringify_baz_second() {
    let x = Baz::Second((2, 3));
    assert_eq!(x.to_string(), "wow");
}

#[test]
fn stringify_quux() {
    let x = Quux::Short;
    assert_eq!(x.to_string(), "short");
}

#[test]
fn foo_from_command() {
    let x = Foo::from_parsed_command(::Command{ name: "two".to_string(), args: Vec::new() });
    assert_eq!(x.unwrap(), Foo::One);
}

#[test]
fn bar_from_command() {
    let x = Bar::from_parsed_command(::Command{ name: "bar".to_string(), args: vec!["1".to_string(), "two".to_string()] });
    assert_eq!(x.unwrap(), Bar::Foo(("1".to_string(), "two".to_string())));
}

#[test]
fn baz_from_command() {
    let x = Baz::from_parsed_command(::Command{ name: "kek".to_string(), args: vec!["1".to_string(), "two".to_string()] });
    assert_eq!(x.unwrap(), Baz::First((1, "two".to_string())));
}

#[test]
fn quux_from_command() {
    let x = Quux::from_parsed_command(::Command{ name: "short".to_string(), args: Vec::new() });
    assert_eq!(x.unwrap(), Quux::Short);
}

#[test]
fn foo_list_commands() {
    let x = Foo::list_commands();
    assert_eq!(x, "two".to_string());
}

#[test]
fn bar_list_commands() {
    let x = Bar::list_commands();
    assert_eq!(x, "bar".to_string());
}

#[test]
fn baz_list_commands() {
    let x = Baz::list_commands();
    assert_eq!(x, "kek\nwow".to_string());
}

#[test]
fn quux_list_commands() {
    let x = Quux::list_commands();
    assert_eq!(x, "long\nshort");
}