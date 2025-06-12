// Answer 0

#[test]
fn test_hir_assertion_start_line() {
    struct Flags {
        unicode: bool,
        multi_line: bool,
    }
    
    struct Context {
        flags: Flags,
    }

    impl Context {
        fn flags(&self) -> &Flags {
            &self.flags
        }
    }

    struct Assertion {
        kind: ast::AssertionKind,
    }

    let ctx = Context {
        flags: Flags {
            unicode: false,
            multi_line: true,
        },
    };

    let asst = Assertion {
        kind: ast::AssertionKind::StartLine,
    };

    let result = ctx.hir_assertion(&asst);
    assert_eq!(result, Ok(Hir::anchor(hir::Anchor::StartLine)));
}

#[test]
fn test_hir_assertion_end_line() {
    struct Flags {
        unicode: bool,
        multi_line: bool,
    }
    
    struct Context {
        flags: Flags,
    }

    impl Context {
        fn flags(&self) -> &Flags {
            &self.flags
        }
    }

    struct Assertion {
        kind: ast::AssertionKind,
    }

    let ctx = Context {
        flags: Flags {
            unicode: false,
            multi_line: true,
        },
    };

    let asst = Assertion {
        kind: ast::AssertionKind::EndLine,
    };

    let result = ctx.hir_assertion(&asst);
    assert_eq!(result, Ok(Hir::anchor(hir::Anchor::EndLine)));
}

#[test]
fn test_hir_assertion_start_text() {
    struct Flags {
        unicode: bool,
        multi_line: bool,
    }
    
    struct Context {
        flags: Flags,
    }

    impl Context {
        fn flags(&self) -> &Flags {
            &self.flags
        }
    }

    struct Assertion {
        kind: ast::AssertionKind,
    }

    let ctx = Context {
        flags: Flags {
            unicode: false,
            multi_line: false,
        },
    };

    let asst = Assertion {
        kind: ast::AssertionKind::StartText,
    };

    let result = ctx.hir_assertion(&asst);
    assert_eq!(result, Ok(Hir::anchor(hir::Anchor::StartText)));
}

#[test]
fn test_hir_assertion_end_text() {
    struct Flags {
        unicode: bool,
        multi_line: bool,
    }
    
    struct Context {
        flags: Flags,
    }

    impl Context {
        fn flags(&self) -> &Flags {
            &self.flags
        }
    }

    struct Assertion {
        kind: ast::AssertionKind,
    }

    let ctx = Context {
        flags: Flags {
            unicode: false,
            multi_line: false,
        },
    };

    let asst = Assertion {
        kind: ast::AssertionKind::EndText,
    };

    let result = ctx.hir_assertion(&asst);
    assert_eq!(result, Ok(Hir::anchor(hir::Anchor::EndText)));
}

#[test]
fn test_hir_assertion_word_boundary_unicode() {
    struct Flags {
        unicode: bool,
        multi_line: bool,
    }
    
    struct Context {
        flags: Flags,
    }

    impl Context {
        fn flags(&self) -> &Flags {
            &self.flags
        }
    }

    struct Assertion {
        kind: ast::AssertionKind,
    }

    let ctx = Context {
        flags: Flags {
            unicode: true,
            multi_line: false,
        },
    };

    let asst = Assertion {
        kind: ast::AssertionKind::WordBoundary,
    };

    let result = ctx.hir_assertion(&asst);
    assert_eq!(result, Ok(Hir::word_boundary(hir::WordBoundary::Unicode)));
}

#[test]
fn test_hir_assertion_word_boundary_ascii() {
    struct Flags {
        unicode: bool,
        multi_line: bool,
    }
    
    struct Context {
        flags: Flags,
    }

    impl Context {
        fn flags(&self) -> &Flags {
            &self.flags
        }
    }

    struct Assertion {
        kind: ast::AssertionKind,
    }

    let ctx = Context {
        flags: Flags {
            unicode: false,
            multi_line: false,
        },
    };

    let asst = Assertion {
        kind: ast::AssertionKind::WordBoundary,
    };

    let result = ctx.hir_assertion(&asst);
    assert_eq!(result, Ok(Hir::word_boundary(hir::WordBoundary::Ascii)));
}

