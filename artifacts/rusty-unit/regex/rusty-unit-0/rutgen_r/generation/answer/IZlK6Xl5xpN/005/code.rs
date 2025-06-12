// Answer 0

#[test]
fn test_cmd_captures_with_valid_input() {
    struct Args {
        expr: String,
    }

    impl Args {
        fn parse_one(&self) -> Result<String> {
            Ok(self.expr.clone())
        }

        fn compiler(&self) -> Compiler {
            Compiler {}
        }
    }

    struct Compiler {}
    
    impl Compiler {
        fn only_utf8(self, _: bool) -> Compiler {
            self
        }

        fn compile(self, exprs: &[String]) -> Result<Program> {
            let captures = exprs.iter().map(|_| None).collect();
            Ok(Program { captures })
        }
    }

    struct Program {
        captures: Vec<Option<String>>,
    }

    let args = Args { expr: String::from("some_pattern") };
    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_captures_with_no_captures() {
    struct Args {
        expr: String,
    }

    impl Args {
        fn parse_one(&self) -> Result<String> {
            Ok(self.expr.clone())
        }

        fn compiler(&self) -> Compiler {
            Compiler {}
        }
    }

    struct Compiler {}

    impl Compiler {
        fn only_utf8(self, _: bool) -> Compiler {
            self
        }

        fn compile(self, exprs: &[String]) -> Result<Program> {
            let captures = exprs.iter().map(|_| None).collect();
            Ok(Program { captures })
        }
    }

    struct Program {
        captures: Vec<Option<String>>,
    }

    let args = Args { expr: String::from("pattern_without_captures") };
    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

