// Answer 0

#[test]
fn test_is_anchored_start_true() {
    struct TestHirInfo {
        bools: u8,
    }

    impl TestHirInfo {
        fn is_anchored_start(&self) -> bool {
            (self.bools & 0b1) > 0
        }
    }

    let info = TestHirInfo { bools: 1 }; // Simulating that it is anchored at the start
    let hir = Hir {
        kind: HirKind::Anchor(Anchor),
        info,
    };

    assert!(hir.is_anchored_start());
}

#[test]
fn test_is_anchored_start_false() {
    struct TestHirInfo {
        bools: u8,
    }

    impl TestHirInfo {
        fn is_anchored_start(&self) -> bool {
            (self.bools & 0b1) > 0
        }
    }

    let info = TestHirInfo { bools: 0 }; // Simulating that it is not anchored at the start
    let hir = Hir {
        kind: HirKind::Alternation(vec![]),
        info,
    };

    assert!(!hir.is_anchored_start());
}

