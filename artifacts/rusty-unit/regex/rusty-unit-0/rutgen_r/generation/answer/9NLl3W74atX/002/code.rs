// Answer 0

#[test]
fn test_cmd_anchors_both_anchored() {
    struct MockArgs;

    impl MockArgs {
        fn new() -> Self {
            MockArgs
        }

        fn parse_one(&self) -> Result<MockExpr, &'static str> {
            Ok(MockExpr::new(true, true))
        }
    }

    struct MockExpr {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl MockExpr {
        fn new(start: bool, end: bool) -> Self {
            MockExpr {
                anchored_start: start,
                anchored_end: end,
            }
        }

        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let args = MockArgs::new();
    assert_eq!(cmd_anchors(&args), Ok(()));
}

#[test]
fn test_cmd_anchors_not_anchored_start() {
    struct MockArgs;

    impl MockArgs {
        fn new() -> Self {
            MockArgs
        }

        fn parse_one(&self) -> Result<MockExpr, &'static str> {
            Ok(MockExpr::new(false, true))
        }
    }

    struct MockExpr {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl MockExpr {
        fn new(start: bool, end: bool) -> Self {
            MockExpr {
                anchored_start: start,
                anchored_end: end,
            }
        }

        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let args = MockArgs::new();
    assert_eq!(cmd_anchors(&args), Ok(()));
}

#[test]
fn test_cmd_anchors_not_anchored_end() {
    struct MockArgs;

    impl MockArgs {
        fn new() -> Self {
            MockArgs
        }

        fn parse_one(&self) -> Result<MockExpr, &'static str> {
            Ok(MockExpr::new(true, false))
        }
    }

    struct MockExpr {
        anchored_start: bool,
        anchored_end: bool,
    }

    impl MockExpr {
        fn new(start: bool, end: bool) -> Self {
            MockExpr {
                anchored_start: start,
                anchored_end: end,
            }
        }

        fn is_anchored_start(&self) -> bool {
            self.anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let args = MockArgs::new();
    assert_eq!(cmd_anchors(&args), Ok(()));
}

