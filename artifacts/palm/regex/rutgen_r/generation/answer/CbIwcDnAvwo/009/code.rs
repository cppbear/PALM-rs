// Answer 0

#[test]
fn test_hir_assertion_end_line_multi_line_false() {
    struct Flags {
        unicode: bool,
        multi_line: bool,
    }
    
    impl Flags {
        fn unicode(&self) -> bool {
            self.unicode
        }
        
        fn multi_line(&self) -> bool {
            self.multi_line
        }
    }
    
    struct Trans {
        allow_invalid_utf8: bool,
    }

    struct TestStruct {
        flags: Flags,
        trans: Trans,
    }
    
    impl TestStruct {
        fn flags(&self) -> &Flags {
            &self.flags
        }
        
        fn trans(&self) -> &Trans {
            &self.trans
        }
        
        fn error(&self, _span: usize, _kind: &str) -> &str {
            "Invalid UTF-8"
        }
        
        fn hir_assertion(&self, asst: &Assertion) -> Result<Hir, &str> {
            let unicode = self.flags().unicode();
            let multi_line = self.flags().multi_line();
            Ok(match asst.kind {
                AssertionKind::StartLine => {
                    Hir::anchor(if multi_line {
                        Anchor::StartLine
                    } else {
                        Anchor::StartText
                    })
                }
                AssertionKind::EndLine => {
                    Hir::anchor(if multi_line {
                        Anchor::EndLine
                    } else {
                        Anchor::EndText
                    })
                }
                AssertionKind::StartText => {
                    Hir::anchor(Anchor::StartText)
                }
                AssertionKind::EndText => {
                    Hir::anchor(Anchor::EndText)
                }
                AssertionKind::WordBoundary => {
                    Hir::word_boundary(if unicode {
                        WordBoundary::Unicode
                    } else {
                        WordBoundary::Ascii
                    })
                }
                AssertionKind::NotWordBoundary => {
                    Hir::word_boundary(if unicode {
                        WordBoundary::UnicodeNegate
                    } else {
                        if !self.trans().allow_invalid_utf8 {
                            return Err(self.error(0, "InvalidUtf8"));
                        }
                        WordBoundary::AsciiNegate
                    })
                }
            })
        }
    }

    enum AssertionKind {
        StartLine,
        EndLine,
        StartText,
        EndText,
        WordBoundary,
        NotWordBoundary,
    }

    struct Assertion {
        kind: AssertionKind,
    }

    struct Hir;
    
    impl Hir {
        fn anchor(anchor: Anchor) -> Hir {
            Hir
        }
        
        fn word_boundary(word_boundary: WordBoundary) -> Hir {
            Hir
        }
    }

    enum Anchor {
        StartLine,
        StartText,
        EndLine,
        EndText,
    }

    enum WordBoundary {
        Unicode,
        Ascii,
        UnicodeNegate,
        AsciiNegate,
    }

    let test_instance = TestStruct {
        flags: Flags { unicode: false, multi_line: false },
        trans: Trans { allow_invalid_utf8: true },
    };

    let assertion = Assertion { kind: AssertionKind::EndLine };
    
    let result = test_instance.hir_assertion(&assertion);
    
    assert!(result.is_ok());
}

