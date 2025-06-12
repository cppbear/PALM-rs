// Answer 0

fn cmd_anchors(args: &Args) -> Result<()> {
    let expr = args.parse_one()?;
    if expr.is_anchored_start() {
        println!("start");
    }
    if expr.is_anchored_end() {
        println!("end");
    }
    Ok(())
}

struct MockArgs {
    expression: Option<MockExpression>,
}

impl MockArgs {
    fn parse_one(&self) -> Result<&MockExpression, &str> {
        self.expression.as_ref().ok_or("No expression")
    }
}

struct MockExpression {
    anchored_start: bool,
    anchored_end: bool,
}

impl MockExpression {
    fn is_anchored_start(&self) -> bool {
        self.anchored_start
    }

    fn is_anchored_end(&self) -> bool {
        self.anchored_end
    }
}

#[test]
fn test_cmd_anchors_neither_anchored() {
    let args = MockArgs {
        expression: Some(MockExpression {
            anchored_start: false,
            anchored_end: false,
        }),
    };
    
    let result = cmd_anchors(&args);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_cmd_anchors_no_expression() {
    let args = MockArgs {
        expression: None,
    };

    let _ = cmd_anchors(&args);
}

#[test]
fn test_cmd_anchors_start_anchored() {
    let args = MockArgs {
        expression: Some(MockExpression {
            anchored_start: true,
            anchored_end: false,
        }),
    };

    let result = cmd_anchors(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_anchors_end_anchored() {
    let args = MockArgs {
        expression: Some(MockExpression {
            anchored_start: false,
            anchored_end: true,
        }),
    };

    let result = cmd_anchors(&args);
    assert!(result.is_ok());
}

