// Answer 0

#[test]
fn test_ascii_class_digit() {
    mod ast {
        pub enum ClassAsciiKind {
            Digit,
        }
    }

    fn ascii_class(kind: &ast::ClassAsciiKind) -> &'static [(char, char)] {
        use ast::ClassAsciiKind::*;

        type T = &'static [(char, char)];
        match *kind {
            Digit => {
                const X: T = &[('0', '9')];
                X
            }
            _ => &[], // Match other cases
        }
    }

    let kind = ast::ClassAsciiKind::Digit;
    let range = ascii_class(&kind);
    assert_eq!(range, &[('0', '9')]);
}

#[test]
fn test_ascii_class_alnum() {
    mod ast {
        pub enum ClassAsciiKind {
            Alnum,
        }
    }

    fn ascii_class(kind: &ast::ClassAsciiKind) -> &'static [(char, char)] {
        use ast::ClassAsciiKind::*;

        type T = &'static [(char, char)];
        match *kind {
            Alnum => {
                const X: T = &[('0', '9'), ('A', 'Z'), ('a', 'z')];
                X
            }
            _ => &[], // Match other cases
        }
    }

    let kind = ast::ClassAsciiKind::Alnum;
    let range = ascii_class(&kind);
    assert_eq!(range, &[('0', '9'), ('A', 'Z'), ('a', 'z')]);
}

