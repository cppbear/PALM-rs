// Answer 0

#[test]
fn test_hir_assertion_word_boundary_ascii() {
    struct TestFlags {
        unicode: bool,
        multi_line: bool,
    }

    impl TestFlags {
        fn unicode(&self) -> bool {
            self.unicode
        }

        fn multi_line(&self) -> bool {
            self.multi_line
        }
    }

    struct Test {
        flags: TestFlags,
        trans: TestTrans,
    }

    impl Test {
        fn flags(&self) -> &TestFlags {
            &self.flags
        }

        fn trans(&self) -> &TestTrans {
            &self.trans
        }

        fn error(&self, _span: &str, _kind: ErrorKind) -> Result<Hir, ()> {
            Err(())
        }
    }

    struct TestTrans {
        allow_invalid_utf8: bool,
    }

    #[derive(Debug)]
    enum AssertionKind {
        WordBoundary,
    }

    struct Assertion {
        kind: AssertionKind,
    }

    #[derive(Debug)]
    enum Hir {
        Anchor(Anchor),
        WordBoundary(WordBoundary),
    }

    #[derive(Debug)]
    enum Anchor {
        StartText,
        EndText,
    }

    #[derive(Debug)]
    enum WordBoundary {
        Ascii,
        Unicode,
        UnicodeNegate,
        AsciiNegate,
    }

    fn hir_assertion(test: &Test, asst: &Assertion) -> Result<Hir, ()> {
        let unicode = test.flags().unicode();
        let multi_line = test.flags().multi_line();
        Ok(match asst.kind {
            AssertionKind::WordBoundary => {
                Hir::WordBoundary(if unicode {
                    WordBoundary::Unicode
                } else {
                    WordBoundary::Ascii
                })
            }
        })
    }

    let test = Test {
        flags: TestFlags { unicode: false, multi_line: false },
        trans: TestTrans { allow_invalid_utf8: true },
    };

    let assertion = Assertion { kind: AssertionKind::WordBoundary };

    let result = hir_assertion(&test, &assertion).unwrap();
    assert_eq!(result, Hir::WordBoundary(WordBoundary::Ascii));
}

