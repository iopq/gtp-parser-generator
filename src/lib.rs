#![allow(non_camel_case_types)]

macro_rules! example {

    (enum $enumer:ident { $($i:ident => $e:expr $(, $m:ident)*;)* } ) => {
        pub enum $enumer {
        $(
            $i (
                $(
                    $m,
                )*
            ),
        )*
        }
        
		
		impl $enumer {
			fn as_str(&self) -> &str {
				match self {
					$(
					
						$i => $e,

					)*
				}
			}
		}
    };
}

//example!(enum Foo { One => "two"; });

example!(enum Bar {
    Foo => "bar", String, String;
});
example!(enum Baz {
   First => "kek", i32, String;
   Second => "wow", i32, i32;
});

#[test]
fn stringify() {
	let x = Baz::First(3, "wew".into());
    assert_eq!(x.as_str(), "kek");
}