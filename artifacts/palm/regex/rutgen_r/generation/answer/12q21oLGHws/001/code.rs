// Answer 0

#[test]
fn test_is_any_anchored_start_with_caret() {
    struct DummyInfo {
        anchored_start: bool,
    }

    impl DummyInfo {
        fn is_any_anchored_start(&self) -> bool {
            self.anchored_start
        }
    }

    struct DummyHir {
        info: DummyInfo,
    }

    let hir = DummyHir {
        info: DummyInfo {
            anchored_start: true,
        },
    };

    assert_eq!(hir.is_any_anchored_start(), true);
}

#[test]
fn test_is_any_anchored_start_with_backslash_a() {
    struct DummyInfo {
        anchored_start: bool,
    }

    impl DummyInfo {
        fn is_any_anchored_start(&self) -> bool {
            self.anchored_start
        }
    }

    struct DummyHir {
        info: DummyInfo,
    }

    let hir = DummyHir {
        info: DummyInfo {
            anchored_start: true,
        },
    };

    assert_eq!(hir.is_any_anchored_start(), true);
}

#[test]
fn test_is_any_anchored_start_without_anchoring() {
    struct DummyInfo {
        anchored_start: bool,
    }

    impl DummyInfo {
        fn is_any_anchored_start(&self) -> bool {
            self.anchored_start
        }
    }

    struct DummyHir {
        info: DummyInfo,
    }

    let hir = DummyHir {
        info: DummyInfo {
            anchored_start: false,
        },
    };

    assert_eq!(hir.is_any_anchored_start(), false);
}

#[test]
fn test_is_any_anchored_start_with_caret_multiline() {
    struct DummyInfo {
        anchored_start: bool,
    }

    impl DummyInfo {
        fn is_any_anchored_start(&self) -> bool {
            self.anchored_start
        }
    }

    struct DummyHir {
        info: DummyInfo,
    }

    let hir = DummyHir {
        info: DummyInfo {
            anchored_start: false,
        },
    };

    assert_eq!(hir.is_any_anchored_start(), false);
}

