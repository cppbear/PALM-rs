// Answer 0

#[test]
fn test_class_bytes_range_end() {
    struct TestInterval;

    impl Interval for TestInterval {
        type Bound = u8;

        fn lower(&self) -> Self::Bound { 0 }
        fn upper(&self) -> Self::Bound { 255 }
        fn set_lower(&mut self, _bound: Self::Bound) {}
        fn set_upper(&mut self, _bound: Self::Bound) {}
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }
    
    let range = ClassBytesRange { start: 0, end: 255 };
    assert_eq!(range.end(), 255);

    let range_min = ClassBytesRange { start: 1, end: 1 };
    assert_eq!(range_min.end(), 1);

    let range_max = ClassBytesRange { start: 254, end: 255 };
    assert_eq!(range_max.end(), 255);
}

