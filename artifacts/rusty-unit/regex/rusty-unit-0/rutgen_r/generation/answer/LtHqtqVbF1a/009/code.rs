// Answer 0

fn fmt_repetition_test_zero_or_one_greedy() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                output: String::new(),
            }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestFormatter {
        wtr: TestWriter,
    }

    impl TestFormatter {
        fn new() -> Self {
            Self {
                wtr: TestWriter::new(),
            }
        }

        fn fmt_repetition(&mut self, ast: &Repetition) -> std::fmt::Result {
            use ast::RepetitionKind::*;
            match ast.op.kind {
                ZeroOrOne if ast.greedy => self.wtr.write_str("?"),
                ZeroOrOne => self.wtr.write_str("??"),
                _ => Ok(()),
            }
        }
    }
    
    mod ast {
        pub struct Op {
            pub kind: RepetitionKind,
        }

        pub struct Repetition {
            pub op: Op,
            pub greedy: bool,
        }

        #[derive(Clone, Copy)]
        pub enum RepetitionKind {
            ZeroOrOne,
        }
    }

    let mut formatter = TestFormatter::new();
    let repetition = ast::Repetition {
        op: ast::Op {
            kind: ast::RepetitionKind::ZeroOrOne,
        },
        greedy: true,
    };

    formatter.fmt_repetition(&repetition).unwrap();
    assert_eq!(formatter.wtr.output, "?");
}

