// Answer 0

#[test]
fn test_ascii_class_lower() {
    mod ast {
        #[derive(Clone, Copy)]
        pub enum ClassAsciiKind {
            Lower,
        }
    }

    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Lower;
    let result = ascii_class(&kind);
    let expected = &[('a', 'z')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_digit() {
    mod ast {
        #[derive(Clone, Copy)]
        pub enum ClassAsciiKind {
            Digit,
        }
    }

    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Digit;
    let result = ascii_class(&kind);
    let expected = &[('0', '9')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_upper() {
    mod ast {
        #[derive(Clone, Copy)]
        pub enum ClassAsciiKind {
            Upper,
        }
    }

    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Upper;
    let result = ascii_class(&kind);
    let expected = &[('A', 'Z')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_word() {
    mod ast {
        #[derive(Clone, Copy)]
        pub enum ClassAsciiKind {
            Word,
        }
    }

    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Word;
    let result = ascii_class(&kind);
    let expected = &[('0', '9'), ('A', 'Z'), ('_', '_'), ('a', 'z')];

    assert_eq!(result, expected);
}

