// Answer 0

#[test]
fn test_start() {
    struct TestInterval;

    impl Clone for TestInterval {
        fn clone(&self) -> Self {
            TestInterval
        }
    }

    impl Copy for TestInterval {}

    impl Default for TestInterval {
        fn default() -> Self {
            TestInterval
        }
    }

    impl Eq for TestInterval {}

    impl PartialEq for TestInterval {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    impl PartialOrd for TestInterval {
        fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
            Some(std::cmp::Ordering::Equal)
        }
    }

    impl Ord for TestInterval {
        fn cmp(&self, _: &Self) -> std::cmp::Ordering {
            std::cmp::Ordering::Equal
        }
    }

    impl Interval for TestInterval {
        type Bound = u8;

        fn lower(&self) -> Self::Bound { 0 }
        fn upper(&self) -> Self::Bound { 255 }
        fn set_lower(&mut self, _: Self::Bound) {}
        fn set_upper(&mut self, _: Self::Bound) {}
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let range = ClassBytesRange::new(10, 20);
    assert_eq!(range.start(), 10);
}

#[test]
fn test_start_boundaries() {
    let range_start = ClassBytesRange::new(u8::MIN, u8::MIN);
    assert_eq!(range_start.start(), u8::MIN);

    let range_end = ClassBytesRange::new(u8::MAX, u8::MAX);
    assert_eq!(range_end.start(), u8::MAX);
}

