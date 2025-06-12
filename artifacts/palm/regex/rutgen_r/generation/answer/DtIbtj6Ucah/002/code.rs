// Answer 0

#[derive(Debug)]
struct DummyError;
#[derive(Debug)]
struct Literal;
impl Literal {
    fn new() -> Self {
        Literal
    }
}

struct Dummy {
    // no state needed for this dummy implementation
}

impl Dummy {
    fn literal_to_char(&self, _ast: &Literal) -> Result<hir::Literal, DummyError> {
        // Simulate returning a Byte variant
        Ok(hir::Literal::Byte(5))
    }

    fn error(&self, _span: usize, _kind: ErrorKind) -> DummyError {
        DummyError
    }
}

mod hir {
    #[derive(Debug)]
    pub enum Literal {
        Byte(u8),
        Unicode(char),
    }
}

#[derive(Debug)]
enum ErrorKind {
    UnicodeNotAllowed,
}

type Result<T> = std::result::Result<T, DummyError>;

#[test]
fn test_class_literal_byte_valid_byte() {
    let dummy = Dummy {};
    let literal = Literal::new();
    let result = dummy.class_literal_byte(&literal);
    assert_eq!(result, Ok(5));
}

#[test]
fn test_class_literal_byte_invalid_unicode() {
    struct DummyUnicode;
    impl DummyUnicode {
        fn literal_to_char(&self, _ast: &Literal) -> Result<hir::Literal, DummyError> {
            // Simulate returning a Unicode variant greater than 0x7F
            Ok(hir::Literal::Unicode('©'))
        }

        fn error(&self, _span: usize, _kind: ErrorKind) -> DummyError {
            DummyError
        }
    }

    let dummy = DummyUnicode {};
    let literal = Literal::new();
    let result = dummy.class_literal_byte(&literal);
    assert!(result.is_err());
}

#[test]
fn test_class_literal_byte_valid_unicode() {
    struct DummyValidUnicode;
    impl DummyValidUnicode {
        fn literal_to_char(&self, _ast: &Literal) -> Result<hir::Literal, DummyError> {
            // Simulate returning a Unicode variant less than or equal to 0x7F
            Ok(hir::Literal::Unicode('A'))
        }

        fn error(&self, _span: usize, _kind: ErrorKind) -> DummyError {
            DummyError
        }
    }

    let dummy = DummyValidUnicode {};
    let literal = Literal::new();
    let result = dummy.class_literal_byte(&literal);
    assert_eq!(result, Ok(65));
}

