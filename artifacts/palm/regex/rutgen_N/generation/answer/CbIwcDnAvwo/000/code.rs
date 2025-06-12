// Answer 0

#[test]
fn test_hir_assertion_start_line() {
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

    struct TestContext {
        flags: TestFlags,
    }

    impl TestContext {
        fn flags(&self) -> &TestFlags {
            &self.flags
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
                _ => unreachable!(),
            })
        }
    }

    let ctx = TestContext {
        flags: TestFlags { unicode: false, multi_line: true },
    };
    
    let assertion = ast::Assertion { kind: ast::AssertionKind::StartLine, span: 0..0 };
    let result = ctx.hir_assertion(&assertion).unwrap();
    assert!(matches!(result, Hir::anchor(hir::Anchor::StartLine)));
}

#[test]
fn test_hir_assertion_end_line() {
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

    struct TestContext {
        flags: TestFlags,
    }

    impl TestContext {
        fn flags(&self) -> &TestFlags {
            &self.flags
        }

        fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {
            let unicode = self.flags().unicode();
            let multi_line = self.flags().multi_line();
            Ok(match asst.kind {
                ast::AssertionKind::EndLine => {
                    Hir::anchor(if multi_line {
                        hir::Anchor::EndLine
                    } else {
                        hir::Anchor::EndText
                    })
                }
                _ => unreachable!(),
            })
        }
    }

    let ctx = TestContext {
        flags: TestFlags { unicode: false, multi_line: true },
    };

    let assertion = ast::Assertion { kind: ast::AssertionKind::EndLine, span: 0..0 };
    let result = ctx.hir_assertion(&assertion).unwrap();
    assert!(matches!(result, Hir::anchor(hir::Anchor::EndLine)));
}

#[test]
fn test_hir_assertion_start_text() {
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

    struct TestContext {
        flags: TestFlags,
    }

    impl TestContext {
        fn flags(&self) -> &TestFlags {
            &self.flags
        }

        fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {
            let unicode = self.flags().unicode();
            let multi_line = self.flags().multi_line();
            Ok(match asst.kind {
                ast::AssertionKind::StartText => {
                    Hir::anchor(hir::Anchor::StartText)
                }
                _ => unreachable!(),
            })
        }
    }

    let ctx = TestContext {
        flags: TestFlags { unicode: false, multi_line: false },
    };

    let assertion = ast::Assertion { kind: ast::AssertionKind::StartText, span: 0..0 };
    let result = ctx.hir_assertion(&assertion).unwrap();
    assert!(matches!(result, Hir::anchor(hir::Anchor::StartText)));
}

#[test]
fn test_hir_assertion_end_text() {
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

    struct TestContext {
        flags: TestFlags,
    }

    impl TestContext {
        fn flags(&self) -> &TestFlags {
            &self.flags
        }

        fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {
            let unicode = self.flags().unicode();
            let multi_line = self.flags().multi_line();
            Ok(match asst.kind {
                ast::AssertionKind::EndText => {
                    Hir::anchor(hir::Anchor::EndText)
                }
                _ => unreachable!(),
            })
        }
    }

    let ctx = TestContext {
        flags: TestFlags { unicode: false, multi_line: false },
    };

    let assertion = ast::Assertion { kind: ast::AssertionKind::EndText, span: 0..0 };
    let result = ctx.hir_assertion(&assertion).unwrap();
    assert!(matches!(result, Hir::anchor(hir::Anchor::EndText)));
}

