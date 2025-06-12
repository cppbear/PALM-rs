// Answer 0

#[test]
fn test_union_with_non_empty_classes() {
    let range1 = ClassBytesRange { start: 0, end: 10 };
    let class_bytes1 = ClassBytes::new(vec![range1]);

    let range2 = ClassBytesRange { start: 5, end: 15 };
    let class_bytes2 = ClassBytes::new(vec![range2]);

    let mut class_bytes_a = class_bytes1.clone();
    class_bytes_a.union(&class_bytes2);
}

#[test]
fn test_union_with_disjoint_ranges() {
    let range1 = ClassBytesRange { start: 0, end: 5 };
    let class_bytes1 = ClassBytes::new(vec![range1]);

    let range2 = ClassBytesRange { start: 6, end: 10 };
    let class_bytes2 = ClassBytes::new(vec![range2]);

    let mut class_bytes_a = class_bytes1.clone();
    class_bytes_a.union(&class_bytes2);
}

#[test]
fn test_union_with_identical_ranges() {
    let range1 = ClassBytesRange { start: 0, end: 10 };
    let class_bytes1 = ClassBytes::new(vec![range1]);

    let class_bytes2 = ClassBytes::new(vec![range1.clone()]);

    let mut class_bytes_a = class_bytes1.clone();
    class_bytes_a.union(&class_bytes2);
}

#[test]
fn test_union_with_overlapping_ranges() {
    let range1 = ClassBytesRange { start: 10, end: 20 };
    let class_bytes1 = ClassBytes::new(vec![range1]);

    let range2 = ClassBytesRange { start: 15, end: 25 };
    let class_bytes2 = ClassBytes::new(vec![range2]);

    let mut class_bytes_a = class_bytes1.clone();
    class_bytes_a.union(&class_bytes2);
}

#[test]
fn test_union_with_full_range() {
    let range1 = ClassBytesRange { start: 0, end: 255 };
    let class_bytes1 = ClassBytes::new(vec![range1]);

    let range2 = ClassBytesRange { start: 100, end: 200 };
    let class_bytes2 = ClassBytes::new(vec![range2]);

    let mut class_bytes_a = class_bytes1.clone();
    class_bytes_a.union(&class_bytes2);
}

#[test]
fn test_union_with_empty_class() {
    let range1 = ClassBytesRange { start: 0, end: 10 };
    let class_bytes1 = ClassBytes::new(vec![range1]);
    
    let class_bytes2 = ClassBytes::empty();

    let mut class_bytes_a = class_bytes1.clone();
    class_bytes_a.union(&class_bytes2);
}

