// Answer 0

#[test]
fn test_intersect_no_ranges_self() {
    let mut class_bytes_self = ClassBytes::empty();
    let class_bytes_other = ClassBytes::new(vec![ClassBytesRange { start: 10, end: 20 }]);
    
    class_bytes_self.intersect(&class_bytes_other);
}

#[test]
fn test_intersect_no_ranges_other() {
    let mut class_bytes_self = ClassBytes::new(vec![ClassBytesRange { start: 10, end: 20 }]);
    let class_bytes_other = ClassBytes::empty();

    class_bytes_self.intersect(&class_bytes_other);
}

#[test]
fn test_intersect_one_range_overlap() {
    let mut class_bytes_self = ClassBytes::new(vec![ClassBytesRange { start: 10, end: 20 }]);
    let class_bytes_other = ClassBytes::new(vec![ClassBytesRange { start: 15, end: 25 }]);

    class_bytes_self.intersect(&class_bytes_other);
}

#[test]
fn test_intersect_one_range_no_overlap() {
    let mut class_bytes_self = ClassBytes::new(vec![ClassBytesRange { start: 10, end: 20 }]);
    let class_bytes_other = ClassBytes::new(vec![ClassBytesRange { start: 21, end: 30 }]);

    class_bytes_self.intersect(&class_bytes_other);
}

#[test]
fn test_intersect_multiple_ranges() {
    let mut class_bytes_self = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 10, end: 15 },
    ]);
    let class_bytes_other = ClassBytes::new(vec![
        ClassBytesRange { start: 3, end: 12 },
        ClassBytesRange { start: 16, end: 20 },
    ]);

    class_bytes_self.intersect(&class_bytes_other);
}

#[test]
fn test_intersect_all_ranges_overlap() {
    let mut class_bytes_self = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 255 },
    ]);
    let class_bytes_other = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 255 },
    ]);

    class_bytes_self.intersect(&class_bytes_other);
}

#[test]
fn test_intersect_sub_range() {
    let mut class_bytes_self = ClassBytes::new(vec![
        ClassBytesRange { start: 5, end: 15 },
    ]);
    let class_bytes_other = ClassBytes::new(vec![
        ClassBytesRange { start: 10, end: 12 },
    ]);

    class_bytes_self.intersect(&class_bytes_other);
}

#[test]
fn test_intersect_edge_cases() {
    let mut class_bytes_self = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 0 },
    ]);
    let class_bytes_other = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 0 },
    ]);

    class_bytes_self.intersect(&class_bytes_other);
}

