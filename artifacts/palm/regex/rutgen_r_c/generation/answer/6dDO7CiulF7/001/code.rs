// Answer 0

#[test]
fn test_intersect_basic() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 0x00, end: 0x7F }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 0x20, end: 0x60 }]);
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[ClassBytesRange { start: 0x20, end: 0x60 }]);
}

#[test]
fn test_intersect_empty() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 0x00, end: 0x7F }]);
    let class_b = ClassBytes::empty();
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[]);
}

#[test]
fn test_intersect_no_overlap() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 0x00, end: 0x0F }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 0x10, end: 0x1F }]);
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[]);
}

#[test]
fn test_intersect_full_overlap() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 0x00, end: 0xFF }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 0x00, end: 0xFF }]);
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[ClassBytesRange { start: 0x00, end: 0xFF }]);
}

#[test]
fn test_intersect_partial_overlap() {
    let mut class_a = ClassBytes::new(vec![ClassBytesRange { start: 0x10, end: 0x30 }]);
    let class_b = ClassBytes::new(vec![ClassBytesRange { start: 0x20, end: 0x40 }]);
    class_a.intersect(&class_b);
    assert_eq!(class_a.ranges(), &[ClassBytesRange { start: 0x20, end: 0x30 }]);
}

