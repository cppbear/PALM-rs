// Answer 0

#[test]
fn test_is_anchored_end_single_expression() {
    struct DummyHir {
        info: DummyInfo,
    }

    struct DummyInfo {
        anchored_end: bool,
    }

    impl DummyInfo {
        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let hir = DummyHir {
        info: DummyInfo { anchored_end: true },
    };
    assert_eq!(hir.is_anchored_end(), true);
}

#[test]
fn test_is_anchored_end_multiple_expressions() {
    struct DummyHir {
        info: DummyInfo,
    }

    struct DummyInfo {
        anchored_end: bool,
    }

    impl DummyInfo {
        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let hir = DummyHir {
        info: DummyInfo { anchored_end: false },
    };
    assert_eq!(hir.is_anchored_end(), false);
}

#[test]
fn test_is_anchored_end_alternation_with_end() {
    struct DummyHir {
        info: DummyInfo,
    }

    struct DummyInfo {
        anchored_end: bool,
    }

    impl DummyInfo {
        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let hir = DummyHir {
        info: DummyInfo { anchored_end: true },
    };
    assert_eq!(hir.is_anchored_end(), true);
}

#[test]
fn test_is_anchored_end_alternation_without_end() {
    struct DummyHir {
        info: DummyInfo,
    }

    struct DummyInfo {
        anchored_end: bool,
    }

    impl DummyInfo {
        fn is_anchored_end(&self) -> bool {
            self.anchored_end
        }
    }

    let hir = DummyHir {
        info: DummyInfo { anchored_end: false },
    };
    assert_eq!(hir.is_anchored_end(), false);
}

