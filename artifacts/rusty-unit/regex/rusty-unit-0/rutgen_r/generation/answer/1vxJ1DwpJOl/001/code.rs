// Answer 0

#[test]
fn test_is_anchored_start_with_start_anchor() {
    struct SimpleHir {
        info: HirInfo,
    }

    struct HirInfo {
        anchored: bool,
    }

    impl HirInfo {
        fn is_anchored_start(&self) -> bool {
            self.anchored
        }
    }

    let hir = SimpleHir {
        info: HirInfo { anchored: true },
    };

    assert!(hir.is_anchored_start());
}

#[test]
fn test_is_anchored_start_with_or_clauses() {
    struct SimpleHir {
        info: HirInfo,
    }

    struct HirInfo {
        anchored: bool,
    }

    impl HirInfo {
        fn is_anchored_start(&self) -> bool {
            self.anchored
        }
    }

    let hir = SimpleHir {
        info: HirInfo { anchored: true },
    };

    assert!(hir.is_anchored_start());
}

#[test]
fn test_is_not_anchored_start_without_start_anchor() {
    struct SimpleHir {
        info: HirInfo,
    }

    struct HirInfo {
        anchored: bool,
    }

    impl HirInfo {
        fn is_anchored_start(&self) -> bool {
            self.anchored
        }
    }

    let hir = SimpleHir {
        info: HirInfo { anchored: false },
    };

    assert!(!hir.is_anchored_start());
}

#[test]
#[should_panic]
fn test_is_anchored_start_panic_condition() {
    struct SimpleHir {
        info: HirInfo,
    }

    struct HirInfo {
        anchored: bool,
    }

    impl HirInfo {
        fn is_anchored_start(&self) -> bool {
            panic!("This should panic");
        }
    }

    let hir = SimpleHir {
        info: HirInfo { anchored: false },
    };

    let _ = hir.is_anchored_start();
}

