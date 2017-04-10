#![allow(non_snake_case)]

macro_rules! example {
    //actually I need to be able to mix and match these so I am matching wrong
    (enum $enumer:ident { $($i:ident => $e:expr $(, $m:ident)+;)* } ) => {
        pub enum $enumer {
        $(
            $i ((
                $(
                    $m,
                )*
            )),
        )*
        }
        
        
        impl $enumer {
            fn as_str(&self) -> &str {
                match self {
                    $(
                    
                        &$enumer::$i(_) => $e,

                    )*
                }
            }
        }
    };
    (enum $enumer:ident { $($i:ident => $e:expr ;)* } ) => {
        pub enum $enumer {
        $(
            $i,
        )*
        }
        
        
        impl $enumer {
            fn as_str(&self) -> &str {
                match self {
                    $(
                    
                        &$enumer::$i => $e,

                    )*
                }
            }
        }
    };
}

example!(enum Foo { One => "two"; });

example!(enum Bar {
    Foo => "bar", String, String;
});
example!(enum Baz {
   First => "kek", i32, String;
   Second => "wow", i32, i32;
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
