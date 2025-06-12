// Answer 0

#[test]
fn test_ranges_empty_intervals() {
    struct TestSet {
        intervals: Vec<ClassBytesRange>,
    }

    impl TestSet {
        fn intervals(&self) -> &[ClassBytesRange] {
            &self.intervals
        }
    }
    
    struct ClassBytesRange;
    
    let test_set = TestSet { intervals: vec![] };
    let result = test_set.intervals();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_ranges_single_interval() {
    struct TestSet {
        intervals: Vec<ClassBytesRange>,
    }

    impl TestSet {
        fn intervals(&self) -> &[ClassBytesRange] {
            &self.intervals
        }
    }
    
    struct ClassBytesRange;
    
    let test_set = TestSet { intervals: vec![ClassBytesRange] };
    let result = test_set.intervals();
    assert_eq!(result.len(), 1);
}

#[test]
fn test_ranges_multiple_intervals() {
    struct TestSet {
        intervals: Vec<ClassBytesRange>,
    }

    impl TestSet {
        fn intervals(&self) -> &[ClassBytesRange] {
            &self.intervals
        }
    }
    
    struct ClassBytesRange;
    
    let test_set = TestSet { intervals: vec![ClassBytesRange, ClassBytesRange, ClassBytesRange] };
    let result = test_set.intervals();
    assert_eq!(result.len(), 3);
}

#[test]
#[should_panic]
fn test_ranges_panic_on_invalid_state() {
    struct TestSet {
        intervals: Vec<ClassBytesRange>,
    }

    impl TestSet {
        fn intervals(&self) -> &[ClassBytesRange] {
            if self.intervals.is_empty() {
                panic!("Invalid state: intervals are empty");
            }
            &self.intervals
        }
    }
    
    struct ClassBytesRange;
    
    let test_set = TestSet { intervals: vec![] };
    let _result = test_set.intervals();
}

