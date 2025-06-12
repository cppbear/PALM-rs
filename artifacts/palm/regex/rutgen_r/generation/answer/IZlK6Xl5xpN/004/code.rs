// Answer 0

fn test_cmd_captures_success() {
    struct Args {
        input: String,
    }

    impl Args {
        fn parse_one(&self) -> Result<String, ()> {
            Ok(self.input.clone())
        }

        fn compiler(&self) -> Compiler {
            Compiler {}
        }
    }

    struct Compiler {}

    impl Compiler {
        fn only_utf8(self, _: bool) -> Self {
            self
        }

        fn compile(self, _: &[String]) -> Result<Program, ()> {
            Ok(Program {
                captures: vec![None, Some("name1".to_string()), None],
            })
        }
    }

    struct Program {
        captures: Vec<Option<String>>,
    }

    let args = Args {
        input: ".*".to_string(),
    };
    
    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

fn test_cmd_captures_only_none() {
    struct Args {
        input: String,
    }

    impl Args {
        fn parse_one(&self) -> Result<String, ()> {
            Ok(self.input.clone())
        }

        fn compiler(&self) -> Compiler {
            Compiler {}
        }
    }

    struct Compiler {}

    impl Compiler {
        fn only_utf8(self, _: bool) -> Self {
            self
        }

        fn compile(self, _: &[String]) -> Result<Program, ()> {
            Ok(Program {
                captures: vec![None],
            })
        }
    }

    struct Program {
        captures: Vec<Option<String>>,
    }

    let args = Args {
        input: ".*".to_string(),
    };
    
    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

fn test_cmd_captures_empty_captures() {
    struct Args {
        input: String,
    }

    impl Args {
        fn parse_one(&self) -> Result<String, ()> {
            Ok(self.input.clone())
        }

        fn compiler(&self) -> Compiler {
            Compiler {}
        }
    }

    struct Compiler {}

    impl Compiler {
        fn only_utf8(self, _: bool) -> Self {
            self
        }

        fn compile(self, _: &[String]) -> Result<Program, ()> {
            Ok(Program {
                captures: vec![],
            })
        }
    }

    struct Program {
        captures: Vec<Option<String>>,
    }

    let args = Args {
        input: ".*".to_string(),
    };
    
    let result = cmd_captures(&args);
    assert!(result.is_ok());
}

#[test]
fn run_tests() {
    test_cmd_captures_success();
    test_cmd_captures_only_none();
    test_cmd_captures_empty_captures();
}

