// Answer 0

#[test]
fn test_ascii_class_alpha() {
    mod ast {
        pub enum ClassAsciiKind {
            Alpha,
        }
    }

    use ast::ClassAsciiKind::*;

    let kind = Alpha;
    let result = ascii_class(&kind);

    let expected: &[(char, char)] = &[('A', 'Z'), ('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_alnum() {
    mod ast {
        pub enum ClassAsciiKind {
            Alnum,
        }
    }

    use ast::ClassAsciiKind::*;

    let kind = Alnum;
    let result = ascii_class(&kind);

    let expected: &[(char, char)] = &[('0', '9'), ('A', 'Z'), ('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_digit() {
    mod ast {
        pub enum ClassAsciiKind {
            Digit,
        }
    }

    use ast::ClassAsciiKind::*;

    let kind = Digit;
    let result = ascii_class(&kind);

    let expected: &[(char, char)] = &[('0', '9')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_lower() {
    mod ast {
        pub enum ClassAsciiKind {
            Lower,
        }
    }

    use ast::ClassAsciiKind::*;

    let kind = Lower;
    let result = ascii_class(&kind);

    let expected: &[(char, char)] = &[('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_upper() {
    mod ast {
        pub enum ClassAsciiKind {
            Upper,
        }
    }

    use ast::ClassAsciiKind::*;

    let kind = Upper;
    let result = ascii_class(&kind);

    let expected: &[(char, char)] = &[('A', 'Z')];
    assert_eq!(result, expected);
}

