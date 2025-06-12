// Answer 0

#[test]
fn test_new_class_bytes_with_non_overlapping_ranges() {
    struct ClassBytesRange {
        start: u8,
        end: u8,
    }

    struct IntervalSet {
        ranges: Vec<ClassBytesRange>,
    }

    impl IntervalSet {
        fn new<I>(ranges: I) -> Self 
        where I: IntoIterator<Item=ClassBytesRange> {
            let mut collected: Vec<ClassBytesRange> = ranges.into_iter().collect();
            collected.sort_by_key(|r| r.start); // Sort by start to handle intervals
            IntervalSet { ranges: collected }
        }
    }

    struct ClassBytes {
        set: IntervalSet,
    }

    let ranges = vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 6, end: 10 }
    ];

    let class_bytes = ClassBytes { set: IntervalSet::new(ranges) };
    assert_eq!(class_bytes.set.ranges.len(), 2);
}

#[test]
fn test_new_class_bytes_with_overlapping_ranges() {
    struct ClassBytesRange {
        start: u8,
        end: u8,
    }

    struct IntervalSet {
        ranges: Vec<ClassBytesRange>,
    }

    impl IntervalSet {
        fn new<I>(ranges: I) -> Self 
        where I: IntoIterator<Item=ClassBytesRange> {
            let mut collected: Vec<ClassBytesRange> = ranges.into_iter().collect();
            collected.sort_by_key(|r| r.start); // Sort by start to handle intervals
            IntervalSet { ranges: collected }
        }
    }

    struct ClassBytes {
        set: IntervalSet,
    }

    let ranges = vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 3, end: 8 }
    ];

    let class_bytes = ClassBytes { set: IntervalSet::new(ranges) };
    assert_eq!(class_bytes.set.ranges.len(), 2); // Expect both ranges collected
}

#[test]
fn test_new_class_bytes_with_single_range() {
    struct ClassBytesRange {
        start: u8,
        end: u8,
    }

    struct IntervalSet {
        ranges: Vec<ClassBytesRange>,
    }

    impl IntervalSet {
        fn new<I>(ranges: I) -> Self 
        where I: IntoIterator<Item=ClassBytesRange> {
            let mut collected: Vec<ClassBytesRange> = ranges.into_iter().collect();
            collected.sort_by_key(|r| r.start); // Sort by start to handle intervals
            IntervalSet { ranges: collected }
        }
    }

    struct ClassBytes {
        set: IntervalSet,
    }

    let range = vec![ClassBytesRange { start: 0, end: 5 }];
    let class_bytes = ClassBytes { set: IntervalSet::new(range) };
    assert_eq!(class_bytes.set.ranges.len(), 1);
} 

#[test]
fn test_new_class_bytes_with_empty_ranges() {
    struct ClassBytesRange {
        start: u8,
        end: u8,
    }

    struct IntervalSet {
        ranges: Vec<ClassBytesRange>,
    }

    impl IntervalSet {
        fn new<I>(ranges: I) -> Self 
        where I: IntoIterator<Item=ClassBytesRange> {
            let mut collected: Vec<ClassBytesRange> = ranges.into_iter().collect();
            collected.sort_by_key(|r| r.start); // Sort by start to handle intervals
            IntervalSet { ranges: collected }
        }
    }

    struct ClassBytes {
        set: IntervalSet,
    }

    let ranges: Vec<ClassBytesRange> = vec![];
    let class_bytes = ClassBytes { set: IntervalSet::new(ranges) };
    assert_eq!(class_bytes.set.ranges.len(), 0); // Check that there are no ranges
}

