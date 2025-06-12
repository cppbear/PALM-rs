// Answer 0

#[test]
fn test_suffixes_unicode_literal() {
    struct MockLiterals {
        data: Vec<u8>,
    }

    impl MockLiterals {
        fn new() -> Self {
            MockLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.data.extend_from_slice(buf);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn add(&mut self, literal: &[u8]) {
            self.data.extend_from_slice(literal);
        }
    }

    let mut literals = MockLiterals::new();
    let unicode_char = 'á'; // Test with a Unicode character
    let expr = Hir::from(HirKind::Literal(hir::Literal::Unicode(unicode_char)));
    suffixes(&expr, &mut literals);
    assert_eq!(literals.data, &['á' as u8, 0, 0, 0]); // Expecting UTF-8 encoded value
}

#[test]
fn test_suffixes_byte_literal() {
    struct MockLiterals {
        data: Vec<u8>,
    }

    impl MockLiterals {
        fn new() -> Self {
            MockLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.data.extend_from_slice(buf);
        }

        fn cut(&mut self) {
            self.data.clear();
        }
    }

    let mut literals = MockLiterals::new();
    let byte_value = 0xFF; // Test with a byte value
    let expr = Hir::from(HirKind::Literal(hir::Literal::Byte(byte_value)));
    suffixes(&expr, &mut literals);
    assert_eq!(literals.data, &[byte_value]); // Expecting the byte value to be added
}

