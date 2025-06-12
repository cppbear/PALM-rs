// Answer 0

#[test]
fn test_class_literal_byte_valid_byte() {
    struct DummyTranslator {
        allow_invalid_utf8: bool,
    }

    impl DummyTranslator {
        fn literal_to_char(&self, ast: &ast::Literal) -> Result<hir::Literal> {
            if let Some(byte) = ast.byte() {
                Ok(hir::Literal::Byte(byte))
            } else {
                Ok(hir::Literal::Unicode(ast.c))
            }
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind,
                pattern: String::new(),
                span,
            }
        }
    }

    let translator = DummyTranslator { allow_invalid_utf8: false };
    let lit = ast::Literal { span: Span { start: 0, end: 1 }, kind: ast::LiteralKind::Byte, c: 'a' }; // example byte literal

    let result = translator.class_literal_byte(&lit);

    assert_eq!(result, Ok(97)); // ASCII value of 'a'
}

#[test]
fn test_class_literal_byte_unicode_allowed() {
    struct DummyTranslator {
        allow_invalid_utf8: bool,
    }

    impl DummyTranslator {
        fn literal_to_char(&self, ast: &ast::Literal) -> Result<hir::Literal> {
            Ok(hir::Literal::Unicode(ast.c))
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind,
                pattern: String::new(),
                span,
            }
        }
    }

    let translator = DummyTranslator { allow_invalid_utf8: false };
    let lit = ast::Literal { span: Span { start: 0, end: 1 }, kind: ast::LiteralKind::Unicode, c: 'A' }; // example unicode character

    let result = translator.class_literal_byte(&lit);

    assert_eq!(result, Ok(65)); // ASCII value of 'A'
}

#[test]
fn test_class_literal_byte_unicode_not_allowed() {
    struct DummyTranslator {
        allow_invalid_utf8: bool,
    }

    impl DummyTranslator {
        fn literal_to_char(&self, ast: &ast::Literal) -> Result<hir::Literal> {
            Ok(hir::Literal::Unicode(ast.c))
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind,
                pattern: String::new(),
                span,
            }
        }
    }

    let translator = DummyTranslator { allow_invalid_utf8: false };
    let lit = ast::Literal { span: Span { start: 0, end: 1 }, kind: ast::LiteralKind::Unicode, c: 'é' }; // example unicode character

    let result = translator.class_literal_byte(&lit);

    assert_eq!(result, Err(translator.error(lit.span, ErrorKind::UnicodeNotAllowed)));
}

#[test]
fn test_class_literal_byte_invalid_utf8() {
    struct DummyTranslator {
        allow_invalid_utf8: bool,
    }

    impl DummyTranslator {
        fn literal_to_char(&self, ast: &ast::Literal) -> Result<hir::Literal> {
            if self.allow_invalid_utf8 {
                Ok(hir::Literal::Unicode(ast.c)) // This would work if invalid UTF-8 was allowed
            } else {
                if ast.c as u32 > 0x7F {
                    Err(self.error(ast.span, ErrorKind::InvalidUtf8))
                } else {
                    Ok(hir::Literal::Unicode(ast.c))
                }
            }
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind,
                pattern: String::new(),
                span,
            }
        }
    }

    let translator = DummyTranslator { allow_invalid_utf8: false };
    let lit = ast::Literal { span: Span { start: 0, end: 1 }, kind: ast::LiteralKind::Unicode, c: '😃' }; // example invalid UTF-8 character

    let result = translator.class_literal_byte(&lit);

    assert_eq!(result, Err(translator.error(lit.span, ErrorKind::UnicodeNotAllowed)));
}

