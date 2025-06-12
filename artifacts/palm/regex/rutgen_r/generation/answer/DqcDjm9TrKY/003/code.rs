// Answer 0

#[test]
fn test_c_repeat_zero_or_one() {
    struct TestStruct;

    impl TestStruct {
        fn c_repeat_zero_or_one(&self, _hir: &hir::Hir, _greedy: bool) -> Result {
            Ok(())
        }
    }

    let rep = hir::Repetition {
        kind: hir::RepetitionKind::ZeroOrOne,
        hir: hir::Hir {},
        greedy: true,
    };
    let mut test_struct = TestStruct;

    let result = test_struct.c_repeat(&rep);
    assert!(result.is_ok());
}

#[test]
fn test_c_repeat_range_at_least() {
    struct TestStruct;

    impl TestStruct {
        fn c_repeat_range_min_or_more(&self, _hir: &hir::Hir, _greedy: bool, _min: usize) -> Result {
            Ok(())
        }
    }

    let rep = hir::Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(3)),
        hir: hir::Hir {},
        greedy: false,
    };
    let mut test_struct = TestStruct;

    let result = test_struct.c_repeat(&rep);
    assert!(result.is_ok());
}

#[test]
fn test_c_repeat_range_bounded() {
    struct TestStruct;

    impl TestStruct {
        fn c_repeat_range(&self, _hir: &hir::Hir, _greedy: bool, _min: usize, _max: usize) -> Result {
            Ok(())
        }
    }

    let rep = hir::Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(2, 5)),
        hir: hir::Hir {},
        greedy: true,
    };
    let mut test_struct = TestStruct;

    let result = test_struct.c_repeat(&rep);
    assert!(result.is_ok());
}

#[test]
fn test_c_repeat_range_exactly() {
    struct TestStruct;

    impl TestStruct {
        fn c_repeat_range(&self, _hir: &hir::Hir, _greedy: bool, _min: usize, _max: usize) -> Result {
            Ok(())
        }
    }

    let rep = hir::Repetition {
        kind: hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(4)),
        hir: hir::Hir {},
        greedy: true,
    };
    let mut test_struct = TestStruct;

    let result = test_struct.c_repeat(&rep);
    assert!(result.is_ok());
}

