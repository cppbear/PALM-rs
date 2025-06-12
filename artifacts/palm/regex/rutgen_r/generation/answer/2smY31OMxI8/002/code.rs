// Answer 0

fn test_c_dotstar_utf8_true() {
    struct Compiled {
        is_utf8: bool,
    }

    impl Compiled {
        fn only_utf8(&self) -> bool {
            self.is_utf8
        }
    }

    struct Hir;

    impl Hir {
        fn any(_: bool) -> Self {
            Hir
        }

        fn repetition(repetition: hir::Repetition) -> Self {
            Hir
        }
    }

    mod hir {
        #[derive(Debug)]
        pub struct Repetition {
            pub kind: RepetitionKind,
            pub greedy: bool,
            pub hir: Box<super::Hir>,
        }

        #[derive(Debug)]
        pub enum RepetitionKind {
            ZeroOrMore,
        }
    }

    struct Tester {
        compiled: Compiled,
    }

    impl Tester {
        fn c(&mut self, _: &Hir) -> Result<Hir, &'static str> {
            Ok(Hir)
        }

        fn c_dotstar(&mut self) -> Result<Hir, &'static str> {
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

    let mut tester = Tester {
        compiled: Compiled { is_utf8: true },
    };

    let result = tester.c_dotstar();
    assert!(result.is_ok());
}

fn test_c_dotstar_non_utf8() {
    struct Compiled {
        is_utf8: bool,
    }

    impl Compiled {
        fn only_utf8(&self) -> bool {
            self.is_utf8
        }
    }

    struct Hir;

    impl Hir {
        fn any(_: bool) -> Self {
            Hir
        }

        fn repetition(repetition: hir::Repetition) -> Self {
            Hir
        }
    }

    mod hir {
        #[derive(Debug)]
        pub struct Repetition {
            pub kind: RepetitionKind,
            pub greedy: bool,
            pub hir: Box<super::Hir>,
        }

        #[derive(Debug)]
        pub enum RepetitionKind {
            ZeroOrMore,
        }
    }

    struct Tester {
        compiled: Compiled,
    }

    impl Tester {
        fn c(&mut self, _: &Hir) -> Result<Hir, &'static str> {
            Ok(Hir)
        }

        fn c_dotstar(&mut self) -> Result<Hir, &'static str> {
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

    let mut tester = Tester {
        compiled: Compiled { is_utf8: false },
    };

    let result = tester.c_dotstar();
    assert!(result.is_ok());
}

