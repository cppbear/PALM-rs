// Answer 0

#[derive(Debug)]
struct Unsupported;

impl std::fmt::Display for Unsupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unsupported Type")
    }
}

#[derive(Debug)]
struct MockSerializer;

impl MockSerializer {
    fn custom(args: std::fmt::Arguments) -> String {
        format!("{}", args)
    }
}

impl MockSerializer {
    fn bad_type(self, what: Unsupported) -> String {
        MockSerializer::custom(format_args!(
            "cannot serialize tagged newtype variant {}::{} containing {}",
            "TypeIdent", "VariantIdent", what
        ))
    }
}

#[test]
fn test_bad_type() {
    let serializer = MockSerializer;
    let unsupported = Unsupported;
    let result = serializer.bad_type(unsupported);
    assert_eq!(result, "cannot serialize tagged newtype variant TypeIdent::VariantIdent containing Unsupported Type");
}

