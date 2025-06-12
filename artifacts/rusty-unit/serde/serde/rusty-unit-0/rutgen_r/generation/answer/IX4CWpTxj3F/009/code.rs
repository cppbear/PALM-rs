// Answer 0

#[test]
fn test_fmt_newtype_struct() {
    use std::fmt;

    struct Unexpected {
        kind: Kind,
    }

    enum Kind {
        NewtypeStruct,
    }

    impl fmt::Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                Kind::NewtypeStruct => formatter.write_str("newtype struct"),
            }
        }
    }

    let unexpected = Unexpected { kind: Kind::NewtypeStruct };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "newtype struct");
}

