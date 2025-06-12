// Answer 0

#[test]
fn test_c_dotstar_utf8_only() {
    struct MockCompiler {
        compiled: Program,
    }

    impl MockCompiler {
        fn new() -> Self {
            MockCompiler {
                compiled: Program {
                    only_utf8: true,
                    ..Program::new()
                },
            }
        }

        fn c(&mut self, _expr: &Hir) -> Result {
            Err(Error::CompiledTooBig(100))
        }

        fn c_dotstar(&mut self) -> Result {
            Ok(if !self.compiled.only_utf8 {
                self.c(&Hir::repetition(hir::Repetition {
                    kind: hir::RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(Hir::any(true)),
                }))?
            } else {
                self.c(&Hir::repetition(hir::Repetition {
                    kind: hir::RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(Hir::any(false)),
                }))?
            })
        }
    }

    let mut compiler = MockCompiler::new();
    let result = compiler.c_dotstar();
    assert!(result.is_err());
    if let Err(Error::CompiledTooBig(size)) = result {
        assert_eq!(size, 100);
    } else {
        panic!("Expected error not received.");
    }
}

#[test]
fn test_c_dotstar_non_utf8() {
    struct MockCompiler {
        compiled: Program,
    }

    impl MockCompiler {
        fn new() -> Self {
            MockCompiler {
                compiled: Program {
                    only_utf8: false,
                    ..Program::new()
                },
            }
        }

        fn c(&mut self, _expr: &Hir) -> Result {
            Ok(Patch {
                hole: Hole::None,
                entry: 0,
            })
        }

        fn c_dotstar(&mut self) -> Result {
            Ok(if !self.compiled.only_utf8 {
                self.c(&Hir::repetition(hir::Repetition {
                    kind: hir::RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(Hir::any(true)),
                }))?
            } else {
                self.c(&Hir::repetition(hir::Repetition {
                    kind: hir::RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(Hir::any(false)),
                }))?
            })
        }
    }

    let mut compiler = MockCompiler::new();
    let result = compiler.c_dotstar();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_c_dotstar_invalid() {
    struct MockCompiler {
        compiled: Program,
    }

    impl MockCompiler {
        fn new() -> Self {
            MockCompiler {
                compiled: Program {
                    only_utf8: true,
                    ..Program::new()
                },
            }
        }

        fn c(&mut self, _expr: &Hir) -> Result {
            panic!("Intentional Panic");
        }

        fn c_dotstar(&mut self) -> Result {
            Ok(if !self.compiled.only_utf8 {
                self.c(&Hir::repetition(hir::Repetition {
                    kind: hir::RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(Hir::any(true)),
                }))?
            } else {
                self.c(&Hir::repetition(hir::Repetition {
                    kind: hir::RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(Hir::any(false)),
                }))?
            })
        }
    }

    let mut compiler = MockCompiler::new();
    let _ = compiler.c_dotstar();
}

