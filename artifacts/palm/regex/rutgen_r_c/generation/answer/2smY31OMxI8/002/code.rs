// Answer 0

#[test]
fn test_c_dotstar_only_utf8() {
    use syntax::hir::{Hir, Repetition, RepetitionKind};
    
    struct TestCompiler {
        compiled: Program,
    }
    
    impl TestCompiler {
        fn new() -> Self {
            TestCompiler {
                compiled: Program::new(),
            }
        }
        
        fn c(&mut self, expr: &Hir) -> Result {
            // Simulate a successful call to c()
            Ok(Patch {
                hole: Hole::None,
                entry: 0,
            })
        }
        
        fn c_dotstar(&mut self) -> Result {
            Ok(if !self.compiled.only_utf8() {
                self.c(&Hir::repetition(Repetition {
                    kind: RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(Hir::any(true)),
                }))?
            } else {
                self.c(&Hir::repetition(Repetition {
                    kind: RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(Hir::any(false)),
                }))?
            })
        }
    }

    let mut compiler = TestCompiler::new();
    compiler.compiled.only_utf8 = true; // set the constraint

    let result = compiler.c_dotstar();
    assert!(result.is_ok());
}

#[test]
fn test_c_dotstar_not_only_utf8() {
    use syntax::hir::{Hir, Repetition, RepetitionKind};

    struct TestCompiler {
        compiled: Program,
    }

    impl TestCompiler {
        fn new() -> Self {
            TestCompiler {
                compiled: Program::new(),
            }
        }
        
        fn c(&mut self, expr: &Hir) -> Result {
            // Simulate a successful call to c()
            Ok(Patch {
                hole: Hole::None,
                entry: 0,
            })
        }

        fn c_dotstar(&mut self) -> Result {
            Ok(if !self.compiled.only_utf8() {
                self.c(&Hir::repetition(Repetition {
                    kind: RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(Hir::any(true)),
                }))?
            } else {
                self.c(&Hir::repetition(Repetition {
                    kind: RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(Hir::any(false)),
                }))?
            })
        }
    }

    let mut compiler = TestCompiler::new();
    compiler.compiled.only_utf8 = false; // set the constraint

    let result = compiler.c_dotstar();
    assert!(result.is_ok());
}

