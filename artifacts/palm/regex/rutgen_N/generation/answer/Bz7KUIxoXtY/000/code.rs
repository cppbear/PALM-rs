// Answer 0

#[derive(Debug)]
struct ClassBytesRange {
    start: u8,
    end: u8,
}

struct TestClass {
    set: TestSet,
}

struct TestSet {
    intervals_value: Vec<ClassBytesRange>,
}

impl TestSet {
    fn intervals(&self) -> &[ClassBytesRange] {
        &self.intervals_value
    }
}

impl TestClass {
    pub fn ranges(&self) -> &[ClassBytesRange] {
        self.set.intervals()
    }
}

#[test]
fn test_ranges_empty() {
    let test_set = TestSet { intervals_value: vec![] };
    let test_class = TestClass { set: test_set };
    
    let result = test_class.ranges();
    assert!(result.is_empty());
}

#[test]
fn test_ranges_single_interval() {
    let test_set = TestSet {
        intervals_value: vec![ClassBytesRange { start: 1, end: 5 }],
    };
    let test_class = TestClass { set: test_set };

    let result = test_class.ranges();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].start, 1);
    assert_eq!(result[0].end, 5);
}

#[test]
fn test_ranges_multiple_intervals() {
    let test_set = TestSet {
        intervals_value: vec![
            ClassBytesRange { start: 1, end: 5 },
            ClassBytesRange { start: 10, end: 15 },
        ],
    };
    let test_class = TestClass { set: test_set };

    let result = test_class.ranges();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].start, 1);
    assert_eq!(result[0].end, 5);
    assert_eq!(result[1].start, 10);
    assert_eq!(result[1].end, 15);
}

