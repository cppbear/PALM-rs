// Answer 0

#[test]
fn test_cmd_captures_empty_expression() {
    struct Args {
        expressions: Vec<String>,
    }

    impl Args {
        fn parse_one(&self) -> Result<String, &'static str> {
            self.expressions.get(0).cloned().ok_or("No expression provided")
        }

        fn compiler(&self) -> Compiler {
            Compiler::new()
        }
    }

    struct Compiler;

    impl Compiler {
        fn new() -> Self {
            Compiler
        }

        fn only_utf8(self, _: bool) -> Self {
            self
        }

        fn compile(self, expressions: &[String]) -> Result<Program, &'static str> {
            let captures = expressions.iter().map(|_| None).collect();
            Ok(Program { captures })
        }
    }

    struct Program {
        captures: Vec<Option<String>>,
    }

    let args = Args {
        expressions: vec![String::new()],
    };

    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_captures_single_named_expression() {
    struct Args {
        expressions: Vec<String>,
    }

    impl Args {
        fn parse_one(&self) -> Result<String, &'static str> {
            self.expressions.get(0).cloned().ok_or("No expression provided")
        }

        fn compiler(&self) -> Compiler {
            Compiler::new()
        }
    }

    struct Compiler;

    impl Compiler {
        fn new() -> Self {
            Compiler
        }

        fn only_utf8(self, _: bool) -> Self {
            self
        }

        fn compile(self, expressions: &[String]) -> Result<Program, &'static str> {
            let captures = expressions
                .iter()
                .map(|e| if e == "named" { Some("name".to_string()) } else { None })
                .collect();
            Ok(Program { captures })
        }
    }

    struct Program {
        captures: Vec<Option<String>>,
    }

    let args = Args {
        expressions: vec!["named".to_string()],
    };

    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_captures_multiple_expressions() {
    struct Args {
        expressions: Vec<String>,
    }

    impl Args {
        fn parse_one(&self) -> Result<String, &'static str> {
            self.expressions.get(0).cloned().ok_or("No expression provided")
        }

        fn compiler(&self) -> Compiler {
            Compiler::new()
        }
    }

    struct Compiler;

    impl Compiler {
        fn new() -> Self {
            Compiler
        }

        fn only_utf8(self, _: bool) -> Self {
            self
        }

        fn compile(self, expressions: &[String]) -> Result<Program, &'static str> {
            let captures = expressions
                .iter()
                .map(|e| if e == "first" { Some("name1".to_string()) } else { None })
                .collect();
            Ok(Program { captures })
        }
    }

    struct Program {
        captures: Vec<Option<String>>,
    }

    let args = Args {
        expressions: vec!["first".to_string(), "second".to_string(), "third".to_string()],
    };

    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

