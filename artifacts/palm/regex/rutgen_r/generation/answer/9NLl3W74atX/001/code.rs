// Answer 0

#[test]
fn test_cmd_anchors_empty_args() {
    struct Args {
        input: String,
    }
    
    impl Args {
        fn new(input: &str) -> Self {
            Args {
                input: input.to_string(),
            }
        }

        fn parse_one(&self) -> Result<Expr, &'static str> {
            if self.input.is_empty() {
                Err("No expression provided")
            } else {
                Ok(Expr::new(&self.input))
            }
        }
    }

    struct Expr {
        value: String,
    }

    impl Expr {
        fn new(value: &str) -> Self {
            Expr {
                value: value.to_string(),
            }
        }

        fn is_anchored_start(&self) -> bool {
            self.value.starts_with('^')
        }

        fn is_anchored_end(&self) -> bool {
            self.value.ends_with('$')
        }
    }

    let args = Args::new("");
    let result = cmd_anchors(&args);
    assert!(result.is_err());
}

#[test]
fn test_cmd_anchors_anchor_start() {
    struct Args {
        input: String,
    }

    impl Args {
        fn new(input: &str) -> Self {
            Args {
                input: input.to_string(),
            }
        }

        fn parse_one(&self) -> Result<Expr, &'static str> {
            Ok(Expr::new(&self.input))
        }
    }

    struct Expr {
        value: String,
    }

    impl Expr {
        fn new(value: &str) -> Self {
            Expr {
                value: value.to_string(),
            }
        }

        fn is_anchored_start(&self) -> bool {
            self.value.starts_with('^')
        }

        fn is_anchored_end(&self) -> bool {
            self.value.ends_with('$')
        }
    }

    let args = Args::new("^start");
    let result = cmd_anchors(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_anchors_anchor_end() {
    struct Args {
        input: String,
    }

    impl Args {
        fn new(input: &str) -> Self {
            Args {
                input: input.to_string(),
            }
        }

        fn parse_one(&self) -> Result<Expr, &'static str> {
            Ok(Expr::new(&self.input))
        }
    }

    struct Expr {
        value: String,
    }

    impl Expr {
        fn new(value: &str) -> Self {
            Expr {
                value: value.to_string(),
            }
        }

        fn is_anchored_start(&self) -> bool {
            self.value.starts_with('^')
        }

        fn is_anchored_end(&self) -> bool {
            self.value.ends_with('$')
        }
    }

    let args = Args::new("end$");
    let result = cmd_anchors(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_anchors_no_anchors() {
    struct Args {
        input: String,
    }

    impl Args {
        fn new(input: &str) -> Self {
            Args {
                input: input.to_string(),
            }
        }

        fn parse_one(&self) -> Result<Expr, &'static str> {
            Ok(Expr::new(&self.input))
        }
    }

    struct Expr {
        value: String,
    }

    impl Expr {
        fn new(value: &str) -> Self {
            Expr {
                value: value.to_string(),
            }
        }

        fn is_anchored_start(&self) -> bool {
            self.value.starts_with('^')
        }

        fn is_anchored_end(&self) -> bool {
            self.value.ends_with('$')
        }
    }

    let args = Args::new("noanchors");
    let result = cmd_anchors(&args);
    assert!(result.is_ok());
}

