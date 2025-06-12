// Answer 0

#[derive(Debug)]
struct Unexpected {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    NewtypeVariant,
}

impl Unexpected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Kind::*;
        match self.kind {
            NewtypeVariant => formatter.write_str("newtype variant"),
        }
    }
}

#[test]
fn test_unexpected_newtype_variant() {
    let unexpected = Unexpected { kind: Kind::NewtypeVariant };
    let mut buffer = std::fmt::Formatter::new();
    
    let result = unexpected.fmt(&mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer.Buffer, "newtype variant");
}

