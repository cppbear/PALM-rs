// Answer 0

#[test]
fn test_symmetric_difference_empty_classes() {
    let mut class1 = ClassBytes::empty();
    let class2 = ClassBytes::empty();
    class1.symmetric_difference(&class2);
}

#[test]
fn test_symmetric_difference_no_overlap() {
    let mut class1 = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 5 }]);
    let class2 = ClassBytes::new(vec![ClassBytesRange { start: 6, end: 10 }]);
    class1.symmetric_difference(&class2);
}

#[test]
fn test_symmetric_difference_partial_overlap() {
    let mut class1 = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 10 }]);
    let class2 = ClassBytes::new(vec![ClassBytesRange { start: 5, end: 15 }]);
    class1.symmetric_difference(&class2);
}

#[test]
fn test_symmetric_difference_full_overlap() {
    let mut class1 = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 10 }]);
    let class2 = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 10 }]);
    class1.symmetric_difference(&class2);
}

#[test]
fn test_symmetric_difference_multiple_ranges() {
    let mut class1 = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 10, end: 15 },
    ]);
    let class2 = ClassBytes::new(vec![
        ClassBytesRange { start: 5, end: 10 },
        ClassBytesRange { start: 15, end: 20 },
    ]);
    class1.symmetric_difference(&class2);
}

#[test]
fn test_symmetric_difference_same_start_end() {
    let mut class1 = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 0 }]);
    let class2 = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 0 }]);
    class1.symmetric_difference(&class2);
}

#[test]
fn test_symmetric_difference_edge_ranges() {
    let mut class1 = ClassBytes::new(vec![ClassBytesRange { start: 250, end: 255 }]);
    let class2 = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 5 }]);
    class1.symmetric_difference(&class2);
}

#[test]
fn test_symmetric_difference_range_start_equal_end() {
    let mut class1 = ClassBytes::new(vec![ClassBytesRange { start: 100, end: 100 }]);
    let class2 = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 100 }]);
    class1.symmetric_difference(&class2);
}

