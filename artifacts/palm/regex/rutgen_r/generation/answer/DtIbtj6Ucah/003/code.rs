// Answer 0

#[test]
fn test_class_literal_byte_valid() {
    struct MockSelf;

    impl MockSelf {
        fn literal_to_char(&self, _ast: &ast::Literal) -> Result<hir::Literal> {
            Ok(hir::Literal::Unicode(0x7F as char)) // Boundary case at 0x7F
        }

        fn error(&self, _span: ast::Span, _kind: ErrorKind) -> String {
            String::from("error")
        }
    }

    let mock_self = MockSelf;
    let ast = ast::Literal::new("Valid"); // Assume a valid Literal instance is created here
    let result = mock_self.class_literal_byte(&ast);
    assert_eq!(result, Ok(0x7F));
}

#[test]
#[should_panic]
fn test_class_literal_byte_unicode_not_allowed() {
    struct MockSelf;

    impl MockSelf {
        fn literal_to_char(&self, _ast: &ast::Literal) -> Result<hir::Literal> {
            Ok(hir::Literal::Unicode('A')) // 'A' is greater than 0x7F
        }

        fn error(&self, _span: ast::Span, _kind: ErrorKind) -> String {
            String::from("error")
        }
    }

    let mock_self = MockSelf;
    let ast = ast::Literal::new("Invalid"); // Assume a valid Literal instance is created here
    let _ = mock_self.class_literal_byte(&ast); // This should panic
}

