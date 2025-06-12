// Answer 0

#[test]
fn test_ascii_class_xdigit() {
    mod ast {
        pub enum ClassAsciiKind {
            Xdigit,
        }
    }

    let kind = ast::ClassAsciiKind::Xdigit;
    let result = ascii_class(&kind);
    let expected = &[('0', '9'), ('A', 'F'), ('a', 'f')];

    assert_eq!(result, expected);
}

