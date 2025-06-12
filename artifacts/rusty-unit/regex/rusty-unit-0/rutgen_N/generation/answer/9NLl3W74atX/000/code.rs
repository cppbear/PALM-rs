// Answer 0

#[test]
fn test_cmd_anchors_start_and_end() {
    struct Args {
        expr: String,
    }

    impl Args {
        fn parse_one(&self) -> Result<Expression, &'static str> {
            Ok(Expression {
                content: self.expr.clone(),
            })
        }
    }

    struct Expression {
        content: String,
    }

    impl Expression {
        fn is_anchored_start(&self) -> bool {
            self.content.starts_with('^')
        }

        fn is_anchored_end(&self) -> bool {
            self.content.ends_with('$')
        }
    }

    let args = Args {
        expr: String::from("^test$"),
    };
    let result = cmd_anchors(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_anchors_neither() {
    struct Args {
        expr: String,
    }

    impl Args {
        fn parse_one(&self) -> Result<Expression, &'static str> {
            Ok(Expression {
                content: self.expr.clone(),
            })
        }
    }

    struct Expression {
        content: String,
    }

    impl Expression {
        fn is_anchored_start(&self) -> bool {
            self.content.starts_with('^')
        }

        fn is_anchored_end(&self) -> bool {
            self.content.ends_with('$')
        }
    }

    let args = Args {
        expr: String::from("test"),
    };
    let result = cmd_anchors(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_anchors_only_start() {
    struct Args {
        expr: String,
    }

    impl Args {
        fn parse_one(&self) -> Result<Expression, &'static str> {
            Ok(Expression {
                content: self.expr.clone(),
            })
        }
    }

    struct Expression {
        content: String,
    }

    impl Expression {
        fn is_anchored_start(&self) -> bool {
            self.content.starts_with('^')
        }

        fn is_anchored_end(&self) -> bool {
            self.content.ends_with('$')
        }
    }

    let args = Args {
        expr: String::from("^test"),
    };
    let result = cmd_anchors(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_anchors_only_end() {
    struct Args {
        expr: String,
    }

    impl Args {
        fn parse_one(&self) -> Result<Expression, &'static str> {
            Ok(Expression {
                content: self.expr.clone(),
            })
        }
    }

    struct Expression {
        content: String,
    }

    impl Expression {
        fn is_anchored_start(&self) -> bool {
            self.content.starts_with('^')
        }

        fn is_anchored_end(&self) -> bool {
            self.content.ends_with('$')
        }
    }

    let args = Args {
        expr: String::from("test$"),
    };
    let result = cmd_anchors(&args);
    assert!(result.is_ok());
}

