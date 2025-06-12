// Answer 0

#[test]
fn test_hir_assertion_start_line_multi_line() {
    struct TestAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let pattern = ".*";
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(true),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
        }),
        allow_invalid_utf8: true,
    };

    let assertion = TestAstAssertion {
        kind: ast::AssertionKind::StartLine,
        span: Span { start: Position(0), end: Position(1) },
    };

    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let result = translator_instance.hir_assertion(&assertion);

    match result {
        Ok(hir) => {
            assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::StartLine));
        }
        Err(_) => panic!("Expected success, but got an error."),
    }
}

#[test]
fn test_hir_assertion_end_line_multi_line() {
    struct TestAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let pattern = ".*";
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(true),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
        }),
        allow_invalid_utf8: true,
    };

    let assertion = TestAstAssertion {
        kind: ast::AssertionKind::EndLine,
        span: Span { start: Position(0), end: Position(1) },
    };

    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let result = translator_instance.hir_assertion(&assertion);

    match result {
        Ok(hir) => {
            assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::EndLine));
        }
        Err(_) => panic!("Expected success, but got an error."),
    }
}

#[test]
fn test_hir_assertion_start_text() {
    struct TestAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let pattern = ".*";
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
        }),
        allow_invalid_utf8: true,
    };

    let assertion = TestAstAssertion {
        kind: ast::AssertionKind::StartText,
        span: Span { start: Position(0), end: Position(1) },
    };

    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let result = translator_instance.hir_assertion(&assertion);

    match result {
        Ok(hir) => {
            assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::StartText));
        }
        Err(_) => panic!("Expected success, but got an error."),
    }
}

#[test]
fn test_hir_assertion_end_text() {
    struct TestAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let pattern = ".*";
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
        }),
        allow_invalid_utf8: true,
    };

    let assertion = TestAstAssertion {
        kind: ast::AssertionKind::EndText,
        span: Span { start: Position(0), end: Position(1) },
    };

    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let result = translator_instance.hir_assertion(&assertion);

    match result {
        Ok(hir) => {
            assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::EndText));
        }
        Err(_) => panic!("Expected success, but got an error."),
    }
}

#[test]
fn test_hir_assertion_word_boundary_unicode() {
    struct TestAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let pattern = ".*";
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
        }),
        allow_invalid_utf8: true,
    };

    let assertion = TestAstAssertion {
        kind: ast::AssertionKind::WordBoundary,
        span: Span { start: Position(0), end: Position(1) },
    };

    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let result = translator_instance.hir_assertion(&assertion);

    match result {
        Ok(hir) => {
            assert_eq!(hir.kind(), &HirKind::WordBoundary(hir::WordBoundary::Unicode));
        }
        Err(_) => panic!("Expected success, but got an error."),
    }
}

#[test]
fn test_hir_assertion_word_boundary_ascii() {
    struct TestAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let pattern = ".*";
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(false),
        }),
        allow_invalid_utf8: true,
    };

    let assertion = TestAstAssertion {
        kind: ast::AssertionKind::WordBoundary,
        span: Span { start: Position(0), end: Position(1) },
    };

    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let result = translator_instance.hir_assertion(&assertion);

    match result {
        Ok(hir) => {
            assert_eq!(hir.kind(), &HirKind::WordBoundary(hir::WordBoundary::Ascii));
        }
        Err(_) => panic!("Expected success, but got an error."),
    }
}

#[test]
fn test_hir_assertion_not_word_boundary_unicode() {
    struct TestAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let pattern = ".*";
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
        }),
        allow_invalid_utf8: true,
    };

    let assertion = TestAstAssertion {
        kind: ast::AssertionKind::NotWordBoundary,
        span: Span { start: Position(0), end: Position(1) },
    };

    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let result = translator_instance.hir_assertion(&assertion);

    match result {
        Ok(hir) => {
            assert_eq!(hir.kind(), &HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate));
        }
        Err(_) => panic!("Expected success, but got an error."),
    }
}

#[test]
fn test_hir_assertion_not_word_boundary_ascii() {
    struct TestAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let pattern = ".*";
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(false),
        }),
        allow_invalid_utf8: false,
    };

    let assertion = TestAstAssertion {
        kind: ast::AssertionKind::NotWordBoundary,
        span: Span { start: Position(0), end: Position(1) },
    };

    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let result = translator_instance.hir_assertion(&assertion);

    match result {
        Err(e) => {
            assert_eq!(e.kind, ErrorKind::InvalidUtf8);
        }
        Ok(_) => panic!("Expected an error for invalid UTF-8, but got success."),
    }
}

