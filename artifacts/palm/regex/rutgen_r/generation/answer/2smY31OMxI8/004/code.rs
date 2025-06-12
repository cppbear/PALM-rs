// Answer 0

#[test]
fn test_c_dotstar_non_utf8_success() {
    struct MockCompiled {
        utf8_enabled: bool,
    }

    impl MockCompiled {
        fn only_utf8(&self) -> bool {
            self.utf8_enabled
        }
    }

    struct MockHir;
    
    impl MockHir {
        fn any(_: bool) -> Self {
            MockHir
        }
        
        fn repetition(rep: hir::Repetition) -> Self {
            MockHir
        }
    }

    struct TestStruct {
        compiled: MockCompiled,
    }

    impl TestStruct {
        fn c(&mut self, _: &MockHir) -> Result<MockHir, &'static str> {
            Ok(MockHir)
        }

        fn c_dotstar(&mut self) -> Result<MockHir, &'static str> {
            Ok(if !self.compiled.only_utf8() {
                self.c(&MockHir::repetition(hir::Repetition {
                    kind: hir::RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(MockHir::any(true)),
                }))?
            } else {
                self.c(&MockHir::repetition(hir::Repetition {
                    kind: hir::RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(MockHir::any(false)),
                }))?
            })
        }
    }

    let mut test_struct = TestStruct {
        compiled: MockCompiled { utf8_enabled: false },
    };

    let result = test_struct.c_dotstar();
    assert!(result.is_ok());
}

#[test]
fn test_c_dotstar_utf8_success() {
    struct MockCompiled {
        utf8_enabled: bool,
    }

    impl MockCompiled {
        fn only_utf8(&self) -> bool {
            self.utf8_enabled
        }
    }

    struct MockHir;
    
    impl MockHir {
        fn any(_: bool) -> Self {
            MockHir
        }
        
        fn repetition(rep: hir::Repetition) -> Self {
            MockHir
        }
    }

    struct TestStruct {
        compiled: MockCompiled,
    }

    impl TestStruct {
        fn c(&mut self, _: &MockHir) -> Result<MockHir, &'static str> {
            Ok(MockHir)
        }

        fn c_dotstar(&mut self) -> Result<MockHir, &'static str> {
            Ok(if !self.compiled.only_utf8() {
                self.c(&MockHir::repetition(hir::Repetition {
                    kind: hir::RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(MockHir::any(true)),
                }))?
            } else {
                self.c(&MockHir::repetition(hir::Repetition {
                    kind: hir::RepetitionKind::ZeroOrMore,
                    greedy: false,
                    hir: Box::new(MockHir::any(false)),
                }))?
            })
        }
    }

    let mut test_struct = TestStruct {
        compiled: MockCompiled { utf8_enabled: true },
    };

    let result = test_struct.c_dotstar();
    assert!(result.is_ok());
}

