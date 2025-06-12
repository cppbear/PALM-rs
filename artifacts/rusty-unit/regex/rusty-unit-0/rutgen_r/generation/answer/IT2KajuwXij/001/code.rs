// Answer 0

#[test]
fn test_is_anchored_end_single_anchor() {
    struct HIR {
        info: Info,
    }

    struct Info {
        anchored: bool,
    }

    impl Info {
        fn is_anchored_end(&self) -> bool {
            self.anchored
        }
    }

    let hir = HIR {
        info: Info { anchored: true },
    };
    assert!(hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_multiple_options_anchored() {
    struct HIR {
        info: Info,
    }

    struct Info {
        anchored: bool,
    }

    impl Info {
        fn is_anchored_end(&self) -> bool {
            self.anchored
        }
    }

    let hir = HIR {
        info: Info { anchored: true },
    };
    assert!(hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_single_non_anchored() {
    struct HIR {
        info: Info,
    }

    struct Info {
        anchored: bool,
    }

    impl Info {
        fn is_anchored_end(&self) -> bool {
            self.anchored
        }
    }

    let hir = HIR {
        info: Info { anchored: false },
    };
    assert!(!hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_multiple_options_non_anchored() {
    struct HIR {
        info: Info,
    }

    struct Info {
        anchored: bool,
    }

    impl Info {
        fn is_anchored_end(&self) -> bool {
            self.anchored
        }
    }

    let hir = HIR {
        info: Info { anchored: false },
    };
    assert!(!hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_mixed_options() {
    struct HIR {
        info: Info,
    }

    struct Info {
        anchored: bool,
    }

    impl Info {
        fn is_anchored_end(&self) -> bool {
            self.anchored
        }
    }

    let hir_a = HIR {
        info: Info { anchored: true },
    };
    let hir_b = HIR {
        info: Info { anchored: false },
    };

    assert!(hir_a.is_anchored_end());
    assert!(!hir_b.is_anchored_end());
}

