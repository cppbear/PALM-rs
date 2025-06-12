// Answer 0

#[test]
fn test_hir_assertion_not_word_boundary() {
    use ast::{self, Assertion, AssertionKind};
    use hir::{self, Hir, ErrorKind};
    use unicode;

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };

    let asst = Assertion {
        span: Span { start: Position::default(), end: Position::default() },
        kind: AssertionKind::NotWordBoundary,
    };

    let translator_instance = TranslatorI::new(&translator, "test_pattern");
    let result = translator_instance.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_not_word_boundary_with_invalid_utf8() {
    use ast::{self, Assertion, AssertionKind};
    use hir::{self, Hir, ErrorKind};
    use unicode;

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };

    let asst = Assertion {
        span: Span { start: Position::default(), end: Position::default() },
        kind: AssertionKind::NotWordBoundary,
    };

    let translator_instance = TranslatorI::new(&translator, "test_pattern");
    let result = translator_instance.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_word_boundary_unicode() {
    use ast::{self, Assertion, AssertionKind};
    use hir::{self, Hir};

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: true,
    };

    let asst = Assertion {
        span: Span { start: Position::default(), end: Position::default() },
        kind: AssertionKind::WordBoundary,
    };

    let translator_instance = TranslatorI::new(&translator, "test_pattern");
    let result = translator_instance.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_not_word_boundary_with_allow_invalid_utf8() {
    use ast::{self, Assertion, AssertionKind};
    use hir::{self, Hir};

    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };

    let asst = Assertion {
        span: Span { start: Position::default(), end: Position::default() },
        kind: AssertionKind::NotWordBoundary,
    };

    let translator_instance = TranslatorI::new(&translator, "test_pattern");
    let result = translator_instance.hir_assertion(&asst);
}

