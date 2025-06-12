// Answer 0

fn test_c_dotstar_utf8_false_err() {
    struct Compiled {
        utf8: bool,
    }

    impl Compiled {
        fn only_utf8(&self) -> bool {
            self.utf8
        }
    }

    struct Hir;

    impl Hir {
        fn any(value: bool) -> Self {
            Hir
        }

        fn repetition(repetition: hir::Repetition) -> Self {
            Hir
        }
    }

    mod hir {
        pub struct Repetition {
            pub kind: RepetitionKind,
            pub greedy: bool,
            pub hir: Box<super::Hir>,
        }

        pub enum RepetitionKind {
            ZeroOrMore,
        }
    }

    struct TestStruct {
        compiled: Compiled,
    }

    impl TestStruct {
        fn c(&self, _hir: &Hir) -> Result<(), String> {
            Err("Error: specific condition not met".to_string())
        }

        fn c_dotstar(&mut self) -> Result<(), String> {
            Ok(if !self.compiled.only_utf8() {
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

    let mut test_instance = TestStruct {
        compiled: Compiled { utf8: true },
    };

    let result = test_instance.c_dotstar();
    assert!(result.is_err());
}

fn test_c_dotstar_utf8_true_err() {
    struct Compiled {
        utf8: bool,
    }

    impl Compiled {
        fn only_utf8(&self) -> bool {
            self.utf8
        }
    }

    struct Hir;

    impl Hir {
        fn any(value: bool) -> Self {
            Hir
        }

        fn repetition(repetition: hir::Repetition) -> Self {
            Hir
        }
    }

    mod hir {
        pub struct Repetition {
            pub kind: RepetitionKind,
            pub greedy: bool,
            pub hir: Box<super::Hir>,
        }

        pub enum RepetitionKind {
            ZeroOrMore,
        }
    }

    struct TestStruct {
        compiled: Compiled,
    }

    impl TestStruct {
        fn c(&self, _hir: &Hir) -> Result<(), String> {
            Err("Error: specific condition not met".to_string())
        }

        fn c_dotstar(&mut self) -> Result<(), String> {
            Ok(if !self.compiled.only_utf8() {
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

    let mut test_instance = TestStruct {
        compiled: Compiled { utf8: false },
    };

    let result = test_instance.c_dotstar();
    assert!(result.is_err());
}

