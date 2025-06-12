// Answer 0

#[test]
fn test_hir_literal_unicode() {
    struct MockTranslator {
        flags: Flags,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            MockTranslator { flags, allow_invalid_utf8 }
        }
        
        fn flags(&self) -> Flags {
            self.flags.clone()
        }

        fn literal_to_char(&self, lit: &ast::Literal) -> Result<hir::Literal> {
            Ok(hir::Literal::Unicode(lit.c))
        }

        fn hir_from_char(&self, span: Span, c: char) -> Result<Hir> {
            Ok(Hir::literal(hir::Literal::Unicode(c)))
        }

        fn hir_from_char_case_insensitive(&self, _span: Span, c: char) -> Result<Hir> {
            Ok(Hir::literal(hir::Literal::Unicode(c)))
        }
    }

    let flags = Flags { case_insensitive: Some(false), ..Default::default() };
    let translator = MockTranslator::new(flags, true);

    let lit = ast::Literal { span: Span { start: 0, end: 1 }, c: 'a' };
    let result = translator.hir_literal(&lit).unwrap();
    assert_eq!(result.kind(), &HirKind::Literal(hir::Literal::Unicode('a')));
}

#[test]
fn test_hir_literal_byte() {
    struct MockTranslator {
        flags: Flags,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            MockTranslator { flags, allow_invalid_utf8 }
        }

        fn flags(&self) -> Flags {
            self.flags.clone()
        }

        fn literal_to_char(&self, lit: &ast::Literal) -> Result<hir::Literal> {
            if lit.c as u8 <= 0x7F {
                Ok(hir::Literal::Byte(lit.c as u8))
            } else {
                Ok(hir::Literal::Unicode(lit.c))
            }
        }

        fn hir_from_char(&self, span: Span, _c: char) -> Result<Hir> {
            Ok(Hir::literal(hir::Literal::Byte(0x7E)))
        }

        fn hir_from_char_case_insensitive(&self, _span: Span, _c: char) -> Result<Hir> {
            Ok(Hir::literal(hir::Literal::Byte(0x7E)))
        }
    }

    let flags = Flags { case_insensitive: Some(false), ..Default::default() };
    let translator = MockTranslator::new(flags, true);

    let lit = ast::Literal { span: Span { start: 0, end: 1 }, c: 'y' };
    let result = translator.hir_literal(&lit).unwrap();
    assert_eq!(result.kind(), &HirKind::Literal(hir::Literal::Byte(0x7E)));
}

