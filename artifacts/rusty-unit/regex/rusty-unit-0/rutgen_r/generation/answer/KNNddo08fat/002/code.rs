// Answer 0

#[test]
fn test_hir_from_char_unicode_not_allowed() {
    // Helper structs to simulate the necessary context
    struct MockFlags {
        unicode: bool,
    }

    impl MockFlags {
        fn unicode(&self) -> bool {
            self.unicode
        }
    }

    struct MockContext {
        flags: MockFlags,
    }

    impl MockContext {
        fn flags(&self) -> &MockFlags {
            &self.flags
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Result<Hir> {
            Err(Error::new("Unicode not allowed".to_string()))
        }

        fn hir_from_char(&self, span: Span, c: char) -> Result<Hir> {
            if !self.flags.unicode() && c.len_utf8() > 1 {
                return Err(self.error(span, ErrorKind::UnicodeNotAllowed));
            }
            Ok(Hir::literal(hir::Literal::Unicode(c)))
        }
    }

    let span = Span::new(0, 1); // Dummy span for testing
    let context = MockContext { flags: MockFlags { unicode: false } };
    let unicode_char = '€'; // A Unicode character that has more than 1 byte

    let result = context.hir_from_char(span, unicode_char);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Unicode not allowed");
}

