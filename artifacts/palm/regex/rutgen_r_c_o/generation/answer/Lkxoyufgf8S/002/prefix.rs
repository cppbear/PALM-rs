// Answer 0

#[test]
fn test_ascii_class_word() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Word;
    let result = ascii_class(&kind);
}

#[test]
fn test_ascii_class_word_empty_range() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Word;
    let result = ascii_class(&kind);
}

