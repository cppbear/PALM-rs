// Answer 0

#[test]
fn test_symmetric_difference_empty_classes() {
    let mut class_a = ClassBytes::empty();
    let class_b = ClassBytes::empty();
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.ranges(), &[]);
}

#[test]
fn test_symmetric_difference_disjoint_classes() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 2 }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 3, end: 4 }]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.ranges(), &[
        ClassBytesRange { start: 1, end: 2 },
        ClassBytesRange { start: 3, end: 4 },
    ]);
}

#[test]
fn test_symmetric_difference_overlapping_classes() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 5 }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 3, end: 7 }]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.ranges(), &[
        ClassBytesRange { start: 1, end: 2 },
        ClassBytesRange { start: 6, end: 7 },
    ]);
}

#[test]
fn test_symmetric_difference_identical_classes() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 3 }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 3 }]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.ranges(), &[]);
}

