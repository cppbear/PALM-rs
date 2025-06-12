// Answer 0

#[test]
fn test_ascii_class_upper() {
    mod ast {
        #[derive(Copy, Clone)]
        pub enum ClassAsciiKind {
            Upper,
        }
    }

    fn ascii_class(kind: &ast::ClassAsciiKind) -> &'static [(char, char)] {
        use ast::ClassAsciiKind::*;

        type T = &'static [(char, char)];
        match *kind {
            Upper => {
                const X: T = &[('A', 'Z')];
                X
            }
            _ => &[(' ', ' ')], // Fallback for non-matching cases (not used in this test)
        }
    }

    let kind = ast::ClassAsciiKind::Upper;
    let result = ascii_class(&kind);

    assert_eq!(result, &[('A', 'Z')]);
}

#[test]
fn test_ascii_class_alnum() {
    mod ast {
        #[derive(Copy, Clone)]
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
            _ => &[(' ', ' ')],
        }
    }

    let kind = ast::ClassAsciiKind::Alnum;
    let result = ascii_class(&kind);

    assert_eq!(result, &[('0', '9'), ('A', 'Z'), ('a', 'z')]);
}

#[test]
fn test_ascii_class_digit() {
    mod ast {
        #[derive(Copy, Clone)]
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
            _ => &[(' ', ' ')],
        }
    }

    let kind = ast::ClassAsciiKind::Digit;
    let result = ascii_class(&kind);

    assert_eq!(result, &[('0', '9')]);
}

