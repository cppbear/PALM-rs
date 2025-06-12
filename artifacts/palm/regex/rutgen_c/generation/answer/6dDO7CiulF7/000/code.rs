// Answer 0

#[test]
fn test_intersect_with_non_overlapping_intervals() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct DummyInterval;

    impl Interval for DummyInterval {
        fn intersect(&self, _other: &Self) -> Option<Self> {
            None
        }
    }

    let mut class_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
    ]);
    let class_b = ClassBytes::new(vec![
        ClassBytesRange { start: 6, end: 10 },
    ]);

    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[ClassBytesRange { start: 0, end: 5 }]);
}

#[test]
fn test_intersect_with_overlapping_intervals() {
    let mut class_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
    ]);
    let class_b = ClassBytes::new(vec![
        ClassBytesRange { start: 3, end: 7 },
    ]);

    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[ClassBytesRange { start: 3, end: 5 }]);
}

#[test]
fn test_intersect_with_identical_intervals() {
    let mut class_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
    ]);
    let class_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
    ]);

    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[ClassBytesRange { start: 0, end: 5 }]);
}

#[test]
fn test_intersect_with_empty_class() {
    let mut class_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
    ]);
    let class_b = ClassBytes::empty();
    
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[]);
}

