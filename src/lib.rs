macro_rules! first {
    ( $a:tt $(, $rest:tt)* ) => ( $a )
}

macro_rules! example {

    (enum $enumer:ident { $($i:ident => $e:tt $( ( $($m:ident),* ) )* ; )* } ) => {
        pub enum $enumer {
        $(
            $i $( ( ( $($m),* ) ) )*,
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
        }
    };
}

example!(enum Foo { One => "two"; });

example!(enum Bar {
    Foo => "bar" (String, String);
});
example!(enum Baz {
   First => "kek" (i32, String);
   Second => "wow" (i32, i32);
});

example!(enum Quux {
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
