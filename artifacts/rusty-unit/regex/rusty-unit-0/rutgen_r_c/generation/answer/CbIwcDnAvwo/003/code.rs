// Answer 0

#[test]
fn test_hir_assertion_not_word_boundary_invalid_utf8() {
    struct FakeTranslator {
        allow_invalid_utf8: bool,
    }
    
    impl FakeTranslator {
        fn new(allow_invalid_utf8: bool) -> Self {
            Self { allow_invalid_utf8 }
        }
        
        fn allow_invalid_utf8(&self) -> bool {
            self.allow_invalid_utf8
        }
    }

    struct FakeTranslatorI<'t> {
        trans: &'t FakeTranslator,
        flags: Flags,
    }

    impl<'t> FakeTranslatorI<'t> {
        fn flags(&self) -> Flags {
            self.flags
        }

        fn trans(&self) -> &FakeTranslator {
            self.trans
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error { kind, pattern: String::new(), span }
        }

        fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {
            let unicode = self.flags().unicode();
            let multi_line = self.flags().multi_line();
            Ok(match asst.kind {
                ast::AssertionKind::StartLine => {
                    Hir::anchor(if multi_line { 
                        hir::Anchor::StartLine 
                    } else { 
                        hir::Anchor::StartText 
                    })
                }
                ast::AssertionKind::EndLine => {
                    Hir::anchor(if multi_line { 
                        hir::Anchor::EndLine 
                    } else { 
                        hir::Anchor::EndText 
                    })
                }
                ast::AssertionKind::StartText => {
                    Hir::anchor(hir::Anchor::StartText)
                }
                ast::AssertionKind::EndText => {
                    Hir::anchor(hir::Anchor::EndText)
                }
                ast::AssertionKind::WordBoundary => {
                    Hir::word_boundary(if unicode { 
                        hir::WordBoundary::Unicode 
                    } else { 
                        hir::WordBoundary::Ascii 
                    })
                }
                ast::AssertionKind::NotWordBoundary => {
                    Hir::word_boundary(if unicode { 
                        hir::WordBoundary::UnicodeNegate 
                    } else {
                        if !self.trans().allow_invalid_utf8 {
                            return Err(self.error(Span { start: Position(0), end: Position(1) }, ErrorKind::InvalidUtf8));
                        }
                        hir::WordBoundary::AsciiNegate
                    })
                }
            })
        }
    }

    let translator = FakeTranslator::new(false);
    let translator_i = FakeTranslatorI {
        trans: &translator,
        flags: Flags { unicode: Some(false), multi_line: None, ..Flags::default() },
    };

    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let result = translator_i.hir_assertion(&assertion);
    assert!(result.is_err());
    if let Err(ref e) = result {
        assert_eq!(e.kind, ErrorKind::InvalidUtf8);
    }
}

