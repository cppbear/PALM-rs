// Answer 0

#[test]
fn test_ascii_class_space() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Space;
    let result = ascii_class(&kind);
}

