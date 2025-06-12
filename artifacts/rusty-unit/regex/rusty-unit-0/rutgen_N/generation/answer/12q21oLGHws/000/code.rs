// Answer 0

#[test]
fn test_is_any_anchored_start_with_caret() {
    struct TestHIR {
        info: TestInfo,
    }

    struct TestInfo {
        anchored: bool,
    }

    impl TestInfo {
        fn is_any_anchored_start(&self) -> bool {
            self.anchored
        }
    }

    let hir = TestHIR {
        info: TestInfo { anchored: true },
    };
    
    assert!(hir.is_any_anchored_start());
}

#[test]
fn test_is_any_anchored_start_without_caret() {
    struct TestHIR {
        info: TestInfo,
    }

    struct TestInfo {
        anchored: bool,
    }

    impl TestInfo {
        fn is_any_anchored_start(&self) -> bool {
            self.anchored
        }
    }

    let hir = TestHIR {
        info: TestInfo { anchored: false },
    };

    assert!(!hir.is_any_anchored_start());
}

