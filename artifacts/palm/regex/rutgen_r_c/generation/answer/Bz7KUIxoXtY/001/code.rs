// Answer 0

#[test]
fn test_ranges_with_single_range() {
    let range = ClassBytesRange { start: 1, end: 10 };
    let intervals = vec![range];
    let class_bytes = ClassBytes {
        set: IntervalSet::new(intervals),
    };

    let output = class_bytes.ranges();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], ClassBytesRange { start: 1, end: 10 });
}

#[test]
fn test_ranges_with_multiple_ranges() {
    let range1 = ClassBytesRange { start: 5, end: 15 };
    let range2 = ClassBytesRange { start: 20, end: 30 };
    let intervals = vec![range1, range2];
    let class_bytes = ClassBytes {
        set: IntervalSet::new(intervals),
    };

    let output = class_bytes.ranges();
    assert_eq!(output.len(), 2);
    assert_eq!(output[0], ClassBytesRange { start: 5, end: 15 });
    assert_eq!(output[1], ClassBytesRange { start: 20, end: 30 });
}

#[test]
fn test_ranges_with_empty_set() {
    let intervals: Vec<ClassBytesRange> = vec![];
    let class_bytes = ClassBytes {
        set: IntervalSet::new(intervals),
    };

    let output = class_bytes.ranges();
    assert_eq!(output.len(), 0);
}

#[test]
fn test_ranges_with_overlapping_ranges() {
    let range1 = ClassBytesRange { start: 1, end: 10 };
    let range2 = ClassBytesRange { start: 5, end: 15 };
    let intervals = vec![range1, range2];
    let class_bytes = ClassBytes {
        set: IntervalSet::new(intervals),
    };

    let output = class_bytes.ranges();
    assert_eq!(output.len(), 2); // Assuming overlaps are allowed in the ranges
    assert_eq!(output[0], ClassBytesRange { start: 1, end: 10 });
    assert_eq!(output[1], ClassBytesRange { start: 5, end: 15 });
}

