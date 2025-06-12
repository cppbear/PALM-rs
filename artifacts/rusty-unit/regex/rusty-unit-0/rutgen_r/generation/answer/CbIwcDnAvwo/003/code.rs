// Answer 0

#[test]
fn test_hir_assertion_not_word_boundary_invalid_utf8() {
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

    impl Trans {
        fn allow_invalid_utf8(&self) -> bool {
            self.allow_invalid_utf8
        }
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

        fn error(&self, _span: usize, _kind: ErrorKind) -> String {
            "Invalid UTF-8".to_string()
        }

        fn hir_assertion(&self, asst: &Assertion) -> Result<String, String> {
            let unicode = self.flags().unicode();
            let multi_line = self.flags().multi_line();
            Ok(match asst.kind {
                AssertionKind::StartLine => {
                    "start_line".to_string()
                }
                AssertionKind::EndLine => {
                    "end_line".to_string()
                }
                AssertionKind::StartText => {
                    "start_text".to_string()
                }
                AssertionKind::EndText => {
                    "end_text".to_string()
                }
                AssertionKind::WordBoundary => {
                    "word_boundary".to_string()
                }
                AssertionKind::NotWordBoundary => {
                    if !self.trans().allow_invalid_utf8 {
                        return Err(self.error(
                            asst.span, ErrorKind::InvalidUtf8));
                    }
                    "not_word_boundary".to_string()
                }
            })
        }
    }

    #[derive(Clone)]
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
        span: usize,
    }

    #[derive(Debug)]
    enum ErrorKind {
        InvalidUtf8,
    }

    let test_struct = TestStruct {
        flags: Flags { unicode: false, multi_line: false },
        trans: Trans { allow_invalid_utf8: false },
    };

    let asst = Assertion {
        kind: AssertionKind::NotWordBoundary,
        span: 0,
    };

    let result = test_struct.hir_assertion(&asst);
    assert_eq!(result, Err("Invalid UTF-8".to_string()));
}

