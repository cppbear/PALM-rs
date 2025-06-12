// Answer 0

#[test]
fn test_new_class_bytes_with_non_overlapping_ranges() {
    struct ClassBytesRange(u8, u8);

    struct ClassBytes {
        set: IntervalSet<ClassBytesRange>,
    }

    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    impl<I> IntervalSet<I> {
        fn new<R>(ranges: R) -> Self 
        where 
            R: IntoIterator<Item=I>
        {
            IntervalSet {
                ranges: ranges.into_iter().collect(),
            }
        }
    }

    let ranges = vec![ClassBytesRange(1, 5), ClassBytesRange(6, 10)];
    let class_bytes = ClassBytes {
        set: IntervalSet::new(ranges),
    };

    assert_eq!(class_bytes.set.ranges.len(), 2);
}

#[test]
fn test_new_class_bytes_with_overlapping_ranges() {
    struct ClassBytesRange(u8, u8);

    struct ClassBytes {
        set: IntervalSet<ClassBytesRange>,
    }

    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    impl<I> IntervalSet<I> {
        fn new<R>(ranges: R) -> Self 
        where 
            R: IntoIterator<Item=I>
        {
            IntervalSet {
                ranges: ranges.into_iter().collect(),
            }
        }
    }

    let ranges = vec![ClassBytesRange(1, 5), ClassBytesRange(4, 10)];
    let class_bytes = ClassBytes {
        set: IntervalSet::new(ranges),
    };
    
    assert_eq!(class_bytes.set.ranges.len(), 2);
} 

#[test]
fn test_new_class_bytes_with_empty_ranges() {
    struct ClassBytesRange(u8, u8);

    struct ClassBytes {
        set: IntervalSet<ClassBytesRange>,
    }

    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    impl<I> IntervalSet<I> {
        fn new<R>(ranges: R) -> Self 
        where 
            R: IntoIterator<Item=I>
        {
            IntervalSet {
                ranges: ranges.into_iter().collect(),
            }
        }
    }

    let ranges: Vec<ClassBytesRange> = vec![];
    let class_bytes = ClassBytes {
        set: IntervalSet::new(ranges),
    };

    assert_eq!(class_bytes.set.ranges.len(), 0);
}

