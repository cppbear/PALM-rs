// Answer 0

#[test]
fn test_cls_byte_count_empty() {
    struct MockClassBytes {
        ranges: Vec<ClassBytesRange>,
    }
    
    impl MockClassBytes {
        fn iter(&self) -> ClassBytesIter {
            ClassBytesIter(IntervalSetIter::new(self.ranges.iter()))
        }
    }
    
    let empty_cls = MockClassBytes { ranges: vec![] };
    assert_eq!(cls_byte_count(&empty_cls), 0);
}

#[test]
fn test_cls_byte_count_single_range() {
    struct MockClassBytes {
        ranges: Vec<ClassBytesRange>,
    }
    
    impl MockClassBytes {
        fn iter(&self) -> ClassBytesIter {
            ClassBytesIter(IntervalSetIter::new(self.ranges.iter()))
        }
    }
    
    let single_range = ClassBytesRange { start: 5, end: 5 };
    let cls = MockClassBytes { ranges: vec![single_range] };
    assert_eq!(cls_byte_count(&cls), 1);
}

#[test]
fn test_cls_byte_count_multiple_ranges() {
    struct MockClassBytes {
        ranges: Vec<ClassBytesRange>,
    }
    
    impl MockClassBytes {
        fn iter(&self) -> ClassBytesIter {
            ClassBytesIter(IntervalSetIter::new(self.ranges.iter()))
        }
    }
    
    let ranges = vec![
        ClassBytesRange { start: 1, end: 3 },
        ClassBytesRange { start: 5, end: 7 },
    ];
    let cls = MockClassBytes { ranges };
    assert_eq!(cls_byte_count(&cls), 6);
}

#[test]
fn test_cls_byte_count_overlapping_ranges() {
    struct MockClassBytes {
        ranges: Vec<ClassBytesRange>,
    }
    
    impl MockClassBytes {
        fn iter(&self) -> ClassBytesIter {
            ClassBytesIter(IntervalSetIter::new(self.ranges.iter()))
        }
    }
    
    let overlapping_ranges = vec![
        ClassBytesRange { start: 2, end: 5 },
        ClassBytesRange { start: 4, end: 6 },
    ];
    let cls = MockClassBytes { ranges: overlapping_ranges };
    assert_eq!(cls_byte_count(&cls), 5); // Counts inclusive ranges
}

